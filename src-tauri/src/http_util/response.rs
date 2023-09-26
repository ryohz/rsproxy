use bytes::Bytes;
use http::header::CONTENT_ENCODING;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri::Manager;
use tokio::sync::mpsc;
use uuid::Uuid;

use super::body::copy_body;
use super::encode::SupportedEncoding;
use super::error::HttpUtilError;
use super::traits::HeaderMapMethods;
use super::traits::VersionMethods;

#[derive(Serialize, Deserialize)]
pub struct ResponseForFront {
    pub headers: String,
    pub body: String,
    pub status: u16,
    pub version: String,
}

impl ResponseForFront {
    pub fn new() -> Self {
        ResponseForFront {
            headers: "".to_string(),
            body: "".to_string(),
            version: "".to_string(),
            status: 500,
        }
    }

    pub async fn from_hyper(
        response: hyper::Response<hyper::Body>,
        pair_id: Option<&Uuid>,
    ) -> Result<Self, HttpUtilError> {
        let (p, s) = match decode_response(response).await {
            Ok(t) => t,
            Err(e) => return Err(HttpUtilError::ResponseFromHyperError(e.to_string())),
        };
        let h = match p.headers.json(pair_id).await {
            Ok(h) => h,
            Err(e) => return Err(HttpUtilError::ResponseFromHyperError(e.to_string())),
        };
        let v = match p.version.to_string() {
            Ok(v) => v,
            Err(e) => return Err(HttpUtilError::ResponseFromHyperError(e.to_string())),
        };
        Ok(Self {
            headers: h,
            version: v,
            status: p.status.as_u16(),
            body: s,
        })
    }

    pub async fn to_hyper(&self) -> Result<hyper::Response<hyper::Body>, HttpUtilError> {
        let h = match hyper::HeaderMap::from_json(self.headers.clone()).await {
            Ok(h) => h,
            Err(e) => return Err(HttpUtilError::ResponseToHyperError(e.to_string())),
        };
        let v = match hyper::Version::from_str(self.version.as_str()) {
            Ok(v) => v,
            Err(e) => return Err(HttpUtilError::ResponseToHyperError(e.to_string())),
        };
        let s = hyper::StatusCode::from_u16(self.status).unwrap();

        let b_bytes = Bytes::from(self.body.clone());
        let s_encoding =
            crate::http_util::encode::SupportedEncoding::from(h.get(CONTENT_ENCODING)).unwrap();
        let e_bytes = match s_encoding.encode(b_bytes) {
            Ok(b) => b,
            Err(e) => return Err(HttpUtilError::ResponseToHyperError(e.to_string())),
        };
        let b = hyper::Body::from(e_bytes);

        let mut rs = hyper::Response::builder().version(v).status(s);
        for (k, v) in h {
            if let Some(k) = k {
                rs = rs.header(k, v);
            }
        }
        match rs.body(b) {
            Ok(rs) => Ok(rs),
            Err(e) => Err(HttpUtilError::ResponseToHyperError(e.to_string())),
        }
    }

    pub async fn send_to_front(&self, app_handle: &AppHandle) -> Result<(), HttpUtilError> {
        let rs_j = match serde_json::to_string(self) {
            Ok(s) => s,
            Err(e) => return Err(HttpUtilError::ResponseSendToFrontError(e.to_string())),
        };
        match app_handle.emit_all("proxy-response", rs_j) {
            Ok(_) => Ok(()),
            Err(e) => Err(HttpUtilError::ResponseSendToFrontError(e.to_string())),
        }
    }

    pub async fn wait_for_modification(
        &self,
        app_handle: &AppHandle,
    ) -> Result<Self, HttpUtilError> {
        let (sender, mut reciever) = mpsc::channel(400);

        app_handle.listen_global("pilot-send-response", move |event| {
            let sender = sender.clone();
            tokio::spawn(async move {
                let rs_str = match event.payload() {
                    Some(s) => s,
                    None => {
                        sender
                            .send(Err(HttpUtilError::ModifiedResponseReceiveError(
                                "received payload is empty".to_string(),
                            )))
                            .await
                            .unwrap();
                        panic!();
                    }
                };
                match serde_json::from_str(rs_str) {
                    Ok(rs) => sender.send(Ok(rs)).await.unwrap(),
                    Err(e) => {
                        sender
                            .send(Err(HttpUtilError::ModifiedResponseReceiveError(
                                e.to_string(),
                            )))
                            .await
                            .unwrap();
                    }
                };
            });
        });

        match reciever.recv().await {
            Some(rs) => rs,
            None => Ok(ResponseForFront::new()),
        }
    }
}

pub async fn copy_response(
    response: hyper::Response<hyper::Body>,
) -> Result<(hyper::Response<hyper::Body>, hyper::Response<hyper::Body>), HttpUtilError> {
    let (p, b) = response.into_parts();

    let h = p.headers;
    let s = p.status;
    let v = p.version;

    let (b1, b2) = match copy_body(b).await {
        Ok(t) => t,
        Err(e) => return Err(HttpUtilError::ResponseCopyError(e.to_string())),
    };

    let rs1 = {
        let mut rs = hyper::Response::builder().status(&s).version(v);
        for (k, v) in &h {
            rs = rs.header(k, v);
        }
        match rs.body(b1) {
            Ok(rs) => rs,
            Err(e) => return Err(HttpUtilError::ResponseCopyError(e.to_string())),
        }
    };

    let rs2 = {
        let mut rs = hyper::Response::builder().status(&s).version(v);
        for (k, v) in &h {
            rs = rs.header(k, v);
        }
        match rs.body(b2) {
            Ok(rs) => rs,
            Err(e) => return Err(HttpUtilError::ResponseCopyError(e.to_string())),
        }
    };

    Ok((rs1, rs2))
}

async fn decode_response(
    response: hyper::Response<hyper::Body>,
) -> Result<(http::response::Parts, String), HttpUtilError> {
    let (parts, body) = response.into_parts();
    let ce = parts.headers.get(CONTENT_ENCODING);
    let se = match SupportedEncoding::from(ce) {
        Ok(s) => s,
        Err(e) => return Err(HttpUtilError::ResponseDecodeError(e.to_string())),
    };
    let b = match se.decode(hyper::body::to_bytes(body).await.unwrap()) {
        Ok(b) => b,
        Err(e) => return Err(HttpUtilError::ResponseDecodeError(e.to_string())),
    };
    let v = Vec::<u8>::from(b);
    let s = match String::from_utf8(v) {
        Ok(s) => s,
        Err(e) => return Err(HttpUtilError::ResponseDecodeError(e.to_string())),
    };
    Ok((parts, s))
}
