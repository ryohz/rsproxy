use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub headers: String,
    pub body: String,
    pub url: String,
    pub method: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub headers: String,
    pub body: String,
    pub url: String,
    pub status: u16,
}

pub type HeadersJson = String;
pub type UrlString = String;
pub type BodyString = String;
pub type MethodString = String;
pub type StatusNumber = u16;
