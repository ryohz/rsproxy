use async_trait::async_trait;
use hyper::{
    header::{HeaderName, HeaderValue},
    HeaderMap,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{
    hash::{Hash, Hasher},
    str::FromStr,
};

#[async_trait]
pub trait FromJson {
    async fn from_json(json_data: String) -> Self;
}

#[async_trait]
impl FromJson for HeaderMap {
    async fn from_json(json_data: String) -> Self {
        let json_headers = json_data;
        let headers_map_from_json: Map<String, Value> =
            serde_json::from_str(&json_headers).unwrap();
        let mut headers = HeaderMap::new();
        for (name, value) in headers_map_from_json {
            let header_name = HeaderName::from_str(name.as_str()).unwrap();
            let header_value = HeaderValue::from_str(value.as_str().unwrap()).unwrap();
            headers.append(header_name, header_value);
        }
        headers
    }
}
