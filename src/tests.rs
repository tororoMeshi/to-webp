#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};
    use bytes::Bytes;
    use std::fs;

    async fn setup_test_app() -> impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    > {
        test::init_service(
            App::new()
                .service(health_check)
                .service(convert_to_webp),
        )
        .await
    }

    #[actix_web::test]
    async fn test_health_check() {
        let app = setup_test_app().await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_unsupported_mime_type() {
        let app = setup_test_app().await;
        
        // HEICとして認識されるようなヘッダーを作成
        let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
        let body = format!(
            "--{0}\r\nContent-Disposition: form-data; name=\"image\"; filename=\"test.heic\"\r\nContent-Type: image/heic\r\n\r\nfake_heic_data\r\n--{0}--\r\n",
            boundary
        );

        let req = test::TestRequest::post()
            .uri("/convert")
            .header("Content-Type", format!("multipart/form-data; boundary={}", boundary))
            .set_payload(Bytes::from(body))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[actix_web::test]
    async fn test_invalid_image_format() {
        let app = setup_test_app().await;
        
        // 不正な画像データを作成
        let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
        let body = format!(
            "--{0}\r\nContent-Disposition: form-data; name=\"image\"; filename=\"test.png\"\r\nContent-Type: image/png\r\n\r\nthis_is_not_a_valid_png\r\n--{0}--\r\n",
            boundary
        );

        let req = test::TestRequest::post()
            .uri("/convert")
            .header("Content-Type", format!("multipart/form-data; boundary={}", boundary))
            .set_payload(Bytes::from(body))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}