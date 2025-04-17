use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{
    error, get, middleware::Logger, post, web, App, Error, HttpResponse, HttpServer, Responder,
};
use futures::{StreamExt, TryStreamExt};
use image::ImageFormat;
use log::{error, info, warn};
use mime_guess;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::Cursor;
use thiserror::Error;
use webp::{Encoder, WebPMemory};

#[cfg(test)]
mod tests;

// 設定定数
const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
const SUPPORTED_FORMATS: [&str; 4] = ["image/png", "image/jpeg", "image/bmp", "image/gif"];
const UNSUPPORTED_FORMATS: [&str; 3] = ["image/heic", "image/heif", "image/hif"];

// エラーレスポンス構造体
#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
    message: String,
    hint: Option<String>,
}

// アプリケーションエラー定義
#[derive(Error, Debug)]
pub enum AppError {
    #[error("画像形式（{0}）は現在サポートされていません")]
    UnsupportedMediaType(String),

    #[error("ファイル形式が不正です")]
    InvalidFileFormat,

    #[error("画像サイズが{0}MBを超えています")]
    PayloadTooLarge(usize),

    #[error("画像の読み込みに失敗しました: {0}")]
    DecodeFailure(String),

    #[error("画像の変換に失敗しました: {0}")]
    ConversionFailed(String),

    #[error("内部サーバーエラー: {0}")]
    InternalError(String),
}

// actix-webのエラー変換実装
impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status_code, error_code, hint) = match self {
            AppError::UnsupportedMediaType(format) => {
                if format.contains("heic") || format.contains("heif") {
                    (
                        actix_web::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                        "unsupported_media_type".to_string(),
                        Some("iPhoneの設定を「互換性優先」にするか、JPEG/PNGに変換してアップロードしてください。".to_string()),
                    )
                } else {
                    (
                        actix_web::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                        "unsupported_media_type".to_string(),
                        Some("サポートされている画像形式（PNG, JPEG, BMP, GIF）を使用してください。".to_string()),
                    )
                }
            }
            AppError::InvalidFileFormat => (
                actix_web::http::StatusCode::BAD_REQUEST,
                "invalid_file_format".to_string(),
                Some("拡張子と中身の形式が一致しているか確認してください。".to_string()),
            ),
            AppError::PayloadTooLarge(size) => (
                actix_web::http::StatusCode::PAYLOAD_TOO_LARGE,
                "payload_too_large".to_string(),
                Some(format!("画像を圧縮または縮小して再度アップロードしてください。最大サイズは{}MBです。", size / 1024 / 1024)),
            ),
            AppError::DecodeFailure(_) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                "decode_failure".to_string(),
                Some("正常な画像か確認してください。".to_string()),
            ),
            AppError::ConversionFailed(_) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "conversion_failed".to_string(),
                Some("しばらくして再試行してください。".to_string()),
            ),
            AppError::InternalError(_) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error".to_string(),
                Some("サーバー側でエラーが発生しました。しばらくして再試行してください。".to_string()),
            ),
        };

        let error_response = ErrorResponse {
            error: error_code,
            message: self.to_string(),
            hint,
        };

        HttpResponse::build(status_code).json(error_response)
    }
}

// MIME検証関数
fn is_supported_mime(mime_type: &str) -> bool {
    SUPPORTED_FORMATS.contains(&mime_type)
}

fn is_unsupported_mime(mime_type: &str) -> bool {
    UNSUPPORTED_FORMATS.iter().any(|&f| mime_type.contains(f))
}

// WebPエンドポイント
#[post("/convert")]
async fn convert_to_webp(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // フィールドを順次処理
    while let Ok(Some(mut field)) = payload.try_next().await {
        // ファイルフィールド名の確認
        if field.name() != "image" {
            continue;
        }

        // Content-TypeとContent-Dispositionの確認
        let content_disposition = field.content_disposition();
        let content_type = match content_disposition.get_filename() {
            Some(filename) => {
                mime_guess::from_path(filename).first_or_text_plain().to_string()
            }
            None => "application/octet-stream".to_string(),
        };

        // サポート外の形式をチェック
        if is_unsupported_mime(&content_type) {
            warn!("Unsupported media type received: {} from IP={}", content_type, "0.0.0.0"); // 実際の実装ではIPアドレスを取得
            return Err(AppError::UnsupportedMediaType(content_type).into());
        }

        // サポート形式かチェック
        if !is_supported_mime(&content_type) {
            return Err(AppError::UnsupportedMediaType(content_type).into());
        }

        // 画像データを読み込み
        let mut bytes = web::BytesMut::new();
        let mut size: usize = 0;

        // データをチャンクで読み込み、サイズ制限チェック
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| {
                error!("Error reading multipart chunk: {}", e);
                AppError::DecodeFailure(e.to_string())
            })?;

            size += data.len();
            if size > MAX_FILE_SIZE {
                return Err(AppError::PayloadTooLarge(MAX_FILE_SIZE).into());
            }

            bytes.extend_from_slice(&data);
        }

        // 画像データを読み込み
        let img = image::load_from_memory(&bytes)
            .map_err(|e| {
                error!("Failed to decode image: {}", e);
                AppError::DecodeFailure(e.to_string())
            })?;

        // WebPエンコーダーでロスレス変換
        let encoder = Encoder::from_image(&img).map_err(|e| {
            error!("WebP encoding error: {}", e);
            AppError::ConversionFailed(e.to_string())
        })?;

        // ロスレス設定
        let webp_data = encoder.encode_lossless();

        // WebPデータをHTTPレスポンスに設定（バイト配列に変換）
        return Ok(HttpResponse::Ok()
            .content_type("image/webp")
            .body(webp_data.to_vec()));
    }

    // 画像フィールドが見つからない場合
    Err(AppError::InvalidFileFormat.into())
}

// ヘルスチェックエンドポイント
#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(web::Json(serde_json::json!({"status": "healthy"})))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 環境変数の読み込み
    dotenv::dotenv().ok();
    env_logger::init();

    // サーバー設定
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port);

    info!("WebP変換APIを開始します: {}", addr);

    // Actixサーバー設定
    HttpServer::new(|| {
        // CORS設定
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(convert_to_webp)
            .service(health_check)
    })
    .bind(addr)?
    .run()
    .await
}