use crate::http_util::traits::FromStr;
use crate::http_util::traits::Json;
use crate::http_util::traits::ToString;
use hyper::{HeaderMap, Method, Uri, Version};
use serde::{Deserialize, Serialize};
use std::str::{self};
use tauri::{AppHandle, Manager};
use tokio::sync::mpsc;

use super::body::copy_body;

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
    pub fn new() -> Self {
        Request {
            headers: "".to_string(),
            url: "".to_string(),
            method: "".to_string(),
            version: "".to_string(),
            body: "".to_string(),
            piloted: false,
        }
    }

    pub async fn from_hyper(request: hyper::Request<hyper::Body>) -> Self {
        let (parts, body) = request.into_parts();

        let headers = parts.headers.to_json().await;
        let url = parts.uri.to_string();
        let method = parts.method.to_string();
        let version = parts.version.to_string();

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

    pub async fn to_hyper(self) -> hyper::Request<hyper::Body> {
        let headers = HeaderMap::from_json(self.headers).await;
        let uri = self.url.parse::<Uri>().unwrap();
        let method = hyper::Method::from_bytes(self.method.as_bytes()).unwrap();
        let version = hyper::Version::from_str(self.version.as_str());
        let body = hyper::Body::from(self.body);

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

    pub async fn send_to_front(&self, app_handle: &AppHandle) {
        let request_json = serde_json::to_string(self).unwrap();
        app_handle.emit_all("proxy-request", request_json).unwrap();
    }

    pub async fn wait_for_modification(&self, app_handle: &AppHandle) -> Self {
        let (sender, mut reciever) = mpsc::channel(400);

        app_handle.listen_global("pilot-send-request", move |event| {
            let sender = sender.clone();
            tokio::spawn(async move {
                let rq_str = event.payload().unwrap();
                let rq: Request = serde_json::from_str(rq_str).unwrap();
                sender.send(rq).await.unwrap();
            });
        });

        if let Some(rq) = reciever.recv().await {
            rq
        } else {
            Request::new()
        }
    }
}

pub async fn copy_request(
    request: hyper::Request<hyper::Body>,
) -> (hyper::Request<hyper::Body>, hyper::Request<hyper::Body>) {
    let (po, b) = request.into_parts();
    let h = po.headers;
    let u = po.uri;
    let m = po.method;
    let v = po.version;

    let (b1, b2) = copy_body(b).await;

    let r1 = {
        let mut r = hyper::Request::builder().uri(&u).method(&m).version(v);
        for (k, v) in &h {
            r = r.header(k, v);
        }
        r.body(b1).unwrap()
    };

    let r2 = {
        let mut r = hyper::Request::builder().uri(&u).method(&m).version(v);
        for (k, v) in &h {
            r = r.header(k, v);
        }
        r.body(b2).unwrap()
    };

    (r1, r2)
}
