use crate::http_util::traits::Json;
use crate::http_util::traits::ToString;
use bytes::Bytes;
use http::header::CONTENT_ENCODING;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri::Manager;
use tokio::sync::mpsc;

use super::body::copy_body;
use super::traits::FromStr;

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub headers: String,
    pub body: String,
    pub status: u16,
    pub version: String,
    pub piloted: bool,
}

impl Response {
    pub fn new() -> Self {
        Response {
            headers: "".to_string(),
            body: "".to_string(),
            version: "".to_string(),
            status: 500,
            piloted: false,
        }
    }

    pub async fn from_hyper(response: hyper::Response<hyper::Body>) -> Self {
        let (p, s) = decode_response(response).await;
        let h = p.headers.to_json().await;
        Self {
            headers: h,
            version: p.version.to_string(),
            status: p.status.as_u16(),
            body: s,
            piloted: false,
        }
    }

    pub async fn to_hyper(&self) -> hyper::Response<hyper::Body> {
        let h = hyper::HeaderMap::from_json(self.headers.clone()).await;
        let v = hyper::Version::from_str(self.version.as_str());
        let s = hyper::StatusCode::from_u16(self.status).unwrap();

        let b_bytes = Bytes::from(self.body.clone());
        let s_encoding =
            crate::http_util::encode::SupportedEncoding::from(h.get(CONTENT_ENCODING)).unwrap();
        let e_bytes = s_encoding.encode(b_bytes);
        let b = hyper::Body::from(e_bytes);

        let mut rs = hyper::Response::builder().version(v).status(s);
        for (k, v) in h {
            if let Some(k) = k {
                rs = rs.header(k, v);
            }
        }
        rs.body(b).unwrap()
    }

    pub async fn send_to_front(&self, app_handle: &AppHandle) {
        let rs_j = serde_json::to_string(self).unwrap();
        app_handle.emit_all("proxy-response", rs_j).unwrap();
    }

    pub async fn wait_for_modification(&self, app_handle: &AppHandle) -> Self {
        let (sender, mut reciever) = mpsc::channel(400);

        app_handle.listen_global("pilot-send-response", move |event| {
            let sender = sender.clone();
            tokio::spawn(async move {
                let rs_str = event.payload().unwrap();
                let rs: Response = serde_json::from_str(rs_str).unwrap();
                sender.send(rs).await.unwrap();
            });
        });

        if let Some(rs) = reciever.recv().await {
            rs
        } else {
            Response::new()
        }
    }
}

pub async fn copy_response(
    response: hyper::Response<hyper::Body>,
) -> (hyper::Response<hyper::Body>, hyper::Response<hyper::Body>) {
    let (p, b) = response.into_parts();

    let h = p.headers;
    let s = p.status;
    let v = p.version;

    let (b1, b2) = copy_body(b).await;

    let rs1 = {
        let mut rs = hyper::Response::builder().status(&s).version(v);
        for (k, v) in &h {
            rs = rs.header(k, v);
        }
        rs.body(b1).unwrap()
    };

    let rs2 = {
        let mut rs = hyper::Response::builder().status(&s).version(v);
        for (k, v) in &h {
            rs = rs.header(k, v);
        }
        rs.body(b2).unwrap()
    };

    (rs1, rs2)
}

async fn decode_response(
    response: hyper::Response<hyper::Body>,
) -> (http::response::Parts, String) {
    let (parts, body) = response.into_parts();
    let ce = parts.headers.get(CONTENT_ENCODING);
    let se = crate::http_util::encode::SupportedEncoding::from(ce).unwrap();
    let b = se.decode(hyper::body::to_bytes(body).await.unwrap());
    let v = Vec::<u8>::from(b);
    let s = String::from_utf8(v).unwrap();
    (parts, s)
}
