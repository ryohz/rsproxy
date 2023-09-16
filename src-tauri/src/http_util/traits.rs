use async_trait::async_trait;
use http::{HeaderMap, Version};
use uuid::Uuid;

use super::error::HttpUtilError;

pub trait VersionMethods {
    fn to_string(&self) -> Result<String, HttpUtilError>;
    fn from_str(s: &str) -> Result<Version, HttpUtilError>;
}

#[async_trait]
pub trait HeaderMapMethods {
    async fn from_json(json_data: String) -> Result<HeaderMap, HttpUtilError>;
    async fn json(&self, id: Option<&Uuid>) -> Result<String, HttpUtilError>;
    fn check_encoding(&self) -> Result<(), HttpUtilError>;
}
