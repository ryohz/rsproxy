use std::str::FromStr;

use crate::http_parser::header::{self, FromJson};
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
