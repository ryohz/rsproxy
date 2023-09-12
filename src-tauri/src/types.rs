use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub headers: String,
    pub body: String,
    pub url: String,
    pub method: String,
    pub piloted: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub headers: String,
    pub body: String,
    pub url: String,
    pub status: u16,
    pub piloted: bool,
}

pub type HeadersJson = String;
pub type UrlString = String;
pub type BodyString = String;
pub type MethodString = String;
pub type StatusNumber = u16;
