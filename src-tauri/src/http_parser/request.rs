use std::str::{self, FromStr};

use crate::http_parser::header::{self, Json};
use hyper::{HeaderMap, Method, Uri, Version};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    // headers parsed as json
    pub headers: String,
    pub url: String,
    pub method: String,
    pub version: String,
    pub body: String,
    pub piloted: bool,
}

impl Request {
    pub async fn from_hyper(request: hyper::Request<hyper::Body>) -> Self {
        let (parts, body) = request.into_parts();

        let headers = parts.headers.to_json().await;
        let url = parts.uri.to_string();
        let method = parts.method.to_string();
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

        let body_bytes = hyper::body::to_bytes(body).await.unwrap();
        let body_vec = Vec::<u8>::from(body_bytes);
        let body = String::from_utf8(body_vec).unwrap();

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

    pub async fn to_hyper(request: Request) -> hyper::Request<hyper::Body> {
        let headers = HeaderMap::from_json(request.headers).await;
        let uri = Uri::from_str(request.url.as_str()).unwrap();
        let method = Method::from_str(request.method.as_str()).unwrap();
        let version = match request.version.as_str() {
            "HTTP/0.9" => Version::HTTP_09,
            "HTTP/1.0" => Version::HTTP_10,
            "HTTP/1.1" => Version::HTTP_11,
            "HTTP/2.0" => Version::HTTP_2,
            "HTTP/3.0" => Version::HTTP_3,
            _ => {
                panic!("Invalid HTTP Version");
            }
        };
        let body = hyper::Body::from(request.body);

        let mut request = hyper::Request::builder();
        for (name, value) in headers {
            request = request.header(name.unwrap(), value);
        }

        request
            .uri(uri)
            .method(method)
            .version(version)
            .body(body)
            .unwrap()
    }
}
