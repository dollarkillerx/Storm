mod error;
mod http;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Sync + Send>>;

pub use async_trait::async_trait;