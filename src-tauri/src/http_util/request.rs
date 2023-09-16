use hyper::{Body, HeaderMap, Method, Uri, Version};
use serde::{Serialize, Serialize};
use std::str::{self};
use tauri::{AppHandle, Manager};
use tokio::sync::mpsc;

use super::body::copy_body;
use super::error::HttpUtilError;
use super::traits::HeaderMapMethods;
use super::traits::VersionMethods;

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

    pub async fn from_hyper(request: hyper::Request<hyper::Body>) -> Result<Self, HttpUtilError> {
        let (p, o_body) = request.into_parts();

        let headers = match p.headers.json().await {
            Ok(h) => h,
            Err(e) => {
                return Err(HttpUtilError::RequestFromHyperError(e.to_string()));
            }
        };
        let url = p.uri.to_string();
        let method = p.method.to_string();
        let version = match p.version.to_string() {
            Ok(v) => v,
            Err(e) => {
                return Err(HttpUtilError::RequestFromHyperError(e.to_string()));
            }
        };

        let body_bytes = match hyper::body::to_bytes(o_body).await {
            Ok(b) => b,
            Err(e) => return Err(HttpUtilError::RequestFromHyperError(e.to_string())),
        };
        let body_vec = Vec::<u8>::from(body_bytes);
        let body = match String::from_utf8(body_vec) {
            Ok(b) => b,
            Err(e) => return Err(HttpUtilError::RequestFromHyperError(e.to_string())),
        };

        let piloted = false;

        Ok(Request {
            headers,
            url,
            method,
            version,
            body,
            piloted,
        })
    }

    pub async fn to_hyper(self) -> Result<hyper::Request<hyper::Body>, HttpUtilError> {
        let headers = match HeaderMap::from_json(self.headers).await {
            Ok(h) => h,
            Err(e) => return Err(HttpUtilError::RequestToHyperError(e.to_string())),
        };
        let uri = match self.url.parse::<Uri>() {
            Ok(u) => u,
            Err(e) => return Err(HttpUtilError::RequestToHyperError(e.to_string())),
        };
        let method = match Method::from_bytes(self.method.as_bytes()) {
            Ok(m) => m,
            Err(e) => return Err(HttpUtilError::RequestToHyperError(e.to_string())),
        };
        let version = match Version::from_str(self.version.as_str()) {
            Ok(v) => v,
            Err(e) => return Err(HttpUtilError::RequestToHyperError(e.to_string())),
        };
        let body = Body::from(self.body);

        let mut request = hyper::Request::builder();
        for (name, value) in headers {
            request = request.header(name.unwrap(), value);
        }

        Ok(request
            .uri(uri)
            .method(method)
            .version(version)
            .body(body)
            .unwrap())
    }

    pub async fn send_to_front(&self, app_handle: &AppHandle) -> Result<(), HttpUtilError> {
        let request_json = match serde_json::to_string(self) {
            Ok(j) => j,
            Err(e) => return Err(HttpUtilError::RequestSendToFrontError(e.to_string())),
        };
        match app_handle.emit_all("proxy-request", request_json) {
            Ok(_) => Ok(()),
            Err(e) => Err(HttpUtilError::RequestSendToFrontError(e.to_string())),
        }
    }

    pub async fn wait_for_modification(
        &self,
        app_handle: &AppHandle,
    ) -> Result<Self, HttpUtilError> {
        let (sender, mut reciever) = mpsc::channel(400);

        app_handle.listen_global("pilot-send-request", move |event| {
            let sender = sender.clone();
            tokio::spawn(async move {
                let rq_str = match event.payload() {
                    Some(r) => r,
                    None => {
                        sender
                            .send(Err(HttpUtilError::ModifiedRequestReceiveError(
                                "received payload is empty".to_string(),
                            )))
                            .await
                            .unwrap();
                        panic!();
                    }
                };
                match serde_json::from_str(rq_str) {
                    Ok(rq) => {
                        sender.send(Ok(rq)).await.unwrap();
                    }
                    Err(e) => {
                        sender
                            .send(Err(HttpUtilError::ModifiedRequestReceiveError(
                                e.to_string(),
                            )))
                            .await
                            .unwrap();
                    }
                };
            });
        });

        match reciever.recv().await {
            Some(rq) => rq,
            None => Ok(Request::new()),
        }
    }

    pub async fn label_pair() {}
}

pub async fn copy_request(
    request: hyper::Request<hyper::Body>,
) -> Result<(hyper::Request<hyper::Body>, hyper::Request<hyper::Body>), HttpUtilError> {
    let (po, b) = request.into_parts();
    let h = po.headers;
    let u = po.uri;
    let m = po.method;
    let v = po.version;

    let (b1, b2) = match copy_body(b).await {
        Ok(b) => b,
        Err(e) => return Err(HttpUtilError::RequestCopyError(e.to_string())),
    };

    let r1 = {
        let mut r = hyper::Request::builder().uri(&u).method(&m).version(v);
        for (k, v) in &h {
            r = r.header(k, v);
        }
        match r.body(b1) {
            Ok(b) => b,
            Err(e) => return Err(HttpUtilError::RequestCopyError(e.to_string())),
        }
    };

    let r2 = {
        let mut r = hyper::Request::builder().uri(&u).method(&m).version(v);
        for (k, v) in &h {
            r = r.header(k, v);
        }
        match r.body(b2) {
            Ok(b) => b,
            Err(e) => return Err(HttpUtilError::RequestCopyError(e.to_string())),
        }
    };

    Ok((r1, r2))
}
