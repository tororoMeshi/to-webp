# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Test Commands
- Build: `cargo build`
- Run: `cargo run`
- Lint: `cargo clippy -- -D warnings`
- Format: `cargo fmt`
- Check: `cargo check`
- Run single test: `cargo test test_name -- --nocapture`
- Format and lint: `./sample/format.sh`

## Code Style Guidelines
- Use Rust 2021 edition standards
- Run `cargo fmt` before committing
- Use `thiserror` for error types, following the enum pattern in GameError
- Prefer strong typing with Serde derives for serialization
- Use async/await for asynchronous operations
- Follow Rust naming conventions: snake_case for variables/functions
- Log errors with appropriate levels (error, info)
- Use Result for error handling with proper error propagation
- Group imports by external crates then standard library
- Document public APIs with comments