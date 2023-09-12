use std::{collections::HashMap, string};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub headers: String,
    pub url: String,
    pub method: String,
    pub version: String,
    pub body: String,
    pub piloted: bool,
}

impl Request {
    pub async fn from_hyper(hyper_request: hyper::Request<hyper::Body>) -> Self {
        let (parts, body) = hyper_request.into_parts();

        let hyper_headers = parts.headers;
        let mut headers_hashmap = HashMap::<String, String>::new();
        for (name, value) in hyper_headers {
            if name.is_some() {
                let name = name.unwrap().to_string();
                let value = value.to_str().unwrap().to_string();
                headers_hashmap.insert(name, value);
            }
        }
        let version = match parts.version {
            hyper::Version::HTTP_09 => "HTTP/0.9",
            hyper::Version::HTTP_10 => "HTTP/1.0",
            hyper::Version::HTTP_11 => "HTTP/1.1",
            hyper::Version::HTTP_2 => "HTTP/2.0",
            hyper::Version::HTTP_3 => "HTTP/3.0",
            _ => {
                panic!("Invalid HTTP Version");
            }
        }
        .to_string();

        let headers = serde_json::to_string(&headers_hashmap).unwrap();
        let url = parts.uri.to_string();
        let method = parts.method.to_string();
        let body = String::from_utf8(hyper::body::to_bytes(body).await.unwrap().to_vec()).unwrap();
        let piloted = false;

        Request {
            headers,
            url,
            method,
            version,
            body,
            piloted,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub headers: String,
    pub url: String,
    pub status: u16,
    pub version: String,
    pub body: String,
    pub piloted: bool,
}

impl Response {
    pub async fn from_request(request: Request) {
        let headers_json = request.headers;
        // let headers_hashmap = serde_json::from_str::<Values>();
        // hyper::header::HOST
    }
}

struct Headers {}
