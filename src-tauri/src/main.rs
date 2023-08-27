// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

struct CurrentExchange {
    headers: String,
    body: String,
}

// impl CurrentExchange {
//     async fn update(&mut self, headers: String, body: String) {
//         self.headers = headers;
//         self.body = body;
//     }
// }

#[tokio::main]
async fn main() {
    // ** shared state for proxy
    let current_request = sync::Arc::new(sync::Mutex::new(CurrentExchange {
        headers: "".to_string(),
        body: "".to_string(),
    }));
    let current_response = sync::Arc::new(sync::Mutex::new(CurrentExchange {
        headers: "".to_string(),
        body: "".to_string(),
    }));

    let pilot_state = sync::Arc::new(sync::Mutex::new(false));

    // test to read shared state
    let current_request1 = sync::Arc::clone(&current_request);
    tokio::spawn(async move {
        let mut prev_headers = String::new();
        loop {
            let current_request = current_request1.lock().unwrap();
            let current_headers = current_request.headers.to_owned();
            if !current_request.headers.is_empty() && prev_headers != current_headers {
                println!("{}", current_request.headers);
                prev_headers = current_request.headers.to_owned();
            }
        }
    });

    // ** proxy server
    let current_request2 = sync::Arc::clone(&current_request);
    tokio::spawn(async move {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        let make_service = hyper::service::make_service_fn(move |_conn| {
            let current_request = current_request2.clone();
            let current_response = current_request.clone();
            let pilot_state = pilot_state.clone();
            async move {
                Ok::<_, convert::Infallible>(hyper::service::service_fn(
                    move |request: hyper::Request<hyper::Body>| {
                        let current_request = current_request.clone();
                        let current_response = current_response.clone();
                        let pilot_state = pilot_state.clone();
                        async move {
                            Ok::<_, convert::Infallible>(
                                proxy_handle(
                                    request,
                                    current_request,
                                    current_response,
                                    pilot_state,
                                )
                                .await
                                .unwrap(),
                            )
                        }
                    },
                ))
            }
        });
        let server = Server::bind(&addr).serve(make_service);

        if let Err(e) = server.await {
            println!("error: {}", e);
        }
    });

    // ** tauri
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use hyper::{body::Body, Server};
use std::io::prelude::*;
use std::net::SocketAddr;
use std::{convert, sync};

// mod error;

// use error::ProxyError;

use thiserror::Error;

// ** ####################################################################################################
// ** Suitable Error struct
// ** ####################################################################################################
#[derive(Error, Debug)]
pub enum ProxyError {
    #[error("hyper error")]
    HyperError(#[from] hyper::Error),
    #[error("hyper http error")]
    HyperHttpError(#[from] hyper::http::Error),
    #[error("failed to parse hyper's uri as reqwest's url")]
    UriParseError(String),
    #[error("invalid http method is given")]
    InvalidMethodError,
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("send request error")]
    SendRequestError(String),
    #[error("response convert error")]
    ResponseConvertError(String),
    #[error("request store error")]
    RequestStoreError(String),
    #[error("serde json error")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("std io error")]
    IoError(#[from] std::io::Error),
    #[error("response store error")]
    ResponseStoreError(String),
    #[error("convert response error")]
    ConvertResponseError(String),
}

// ** ####################################################################################################
// ** Handling proxy block
// ** ####################################################################################################
async fn proxy_handle(
    request: hyper::Request<hyper::Body>,
    current_request: sync::Arc<sync::Mutex<CurrentExchange>>,
    current_response: sync::Arc<sync::Mutex<CurrentExchange>>,
    pilot_state: sync::Arc<sync::Mutex<bool>>,
) -> Result<hyper::Response<hyper::Body>, ProxyError> {
    // ** convert hyper request to reqwest request
    let reqw_request = hyper2reqwest(request).await.unwrap();

    let mut exchange = Exchange::new(reqw_request, false);

    // ** get request headers and body, and set them to exchange's property
    exchange.fetch_request().await.unwrap();
    exchange.set_current_requeste(current_request).await;
    // let (request_body, request_headers) = exchange.get_request().await?;
    // let mut current_request = current_request.lock().unwrap();
    // current_request.body = request_body;
    // current_request.headers = request_headers;

    // ** communicate with remote host, get response from it, and set headers and body to exchange's property
    exchange.fetch_response().await.unwrap();
    // let (response_body, response_headers) = exchange.get_response().await?;
    // let mut current_response = current_response.lock().unwrap();
    // current_response.headers = response_headers;
    // current_response.body = response_body;

    if let Some(reqw_response) = exchange.response {
        let hyper_response = reqwest2hyper(reqw_response).await.unwrap();
        Ok(hyper_response)
    } else {
        Err(ProxyError::ConvertResponseError(
            "response isn't set".to_string(),
        ))
    }
}

struct Exchange {
    request: reqwest::Request,
    response: Option<reqwest::Response>,
    request_body: Option<String>,
    response_body: Option<String>,
    request_headers: Option<String>,
    response_headers: Option<String>,
    pilot_flag: bool,
}

impl Exchange {
    fn new(request: reqwest::Request, pilot_flag: bool) -> Self {
        Exchange {
            request,
            response: None,
            request_headers: None,
            response_headers: None,
            request_body: None,
            response_body: None,
            pilot_flag,
        }
    }

    async fn fetch_request(&mut self) -> Result<(), ProxyError> {
        // ** headers
        let headers = self.request.headers();
        let mut headers_hashmap = std::collections::HashMap::<String, String>::new();
        for (name, value) in headers {
            let name = name.to_string();
            let value = value.to_str().unwrap().to_string();
            headers_hashmap.insert(name, value);
        }
        let headers_json = serde_json::to_string(&headers_hashmap)?;
        self.request_headers = Some(headers_json);

        // ** body
        let request_body = self.request.body().unwrap();
        let mut request_body = request_body.as_bytes().unwrap();
        let mut body = String::new();
        let _ = request_body.read_to_string(&mut body)?;
        self.request_body = Some(body);

        Ok(())
    }

    async fn fetch_response(&mut self) -> Result<(), ProxyError> {
        // ** communicate with remote server
        let client = reqwest::Client::builder()
            .gzip(true)
            .deflate(true)
            .build()?;
        let response = client
            .execute(self.request.try_clone().expect("failed to clone request"))
            .await?;

        // ** headers
        let headers = response.headers();
        let mut headers_hashmap = std::collections::HashMap::<String, String>::new();
        for (name, value) in headers {
            let name = name.to_string();
            let value = value.to_str().unwrap().to_string();
            headers_hashmap.insert(name, value);
        }
        let headers_json = serde_json::to_string(&headers_hashmap)?;
        self.response_headers = Some(headers_json);

        // ** body
        let body_text = response.text().await?;
        self.response_body = Some(body_text.to_owned());

        // ** recreate a response from body_text
        let hyper_body = hyper::Body::from(body_text.as_bytes().to_owned());
        let hyper_response = hyper::Response::builder().body(hyper_body)?;
        let response = reqwest::Response::from(hyper_response);

        // ** response
        self.response = Some(response);

        Ok(())
    }

    fn get_request(&self) -> Result<(String, String), ProxyError> {
        if let Some(header) = self.request_headers.clone() {
            if let Some(body) = self.request_body.clone() {
                let header = header;
                let body = body;
                Ok((header, body))
            } else {
                Err(ProxyError::RequestStoreError(
                    "request body isn't set".to_string(),
                ))
            }
        } else {
            Err(ProxyError::RequestStoreError(
                "request header isn't set".to_string(),
            ))
        }
    }

    fn get_response(&self) -> Result<(String, String), ProxyError> {
        if let Some(header) = self.response_headers.clone() {
            if let Some(body) = self.response_body.clone() {
                let header = header;
                let body = body;
                Ok((header, body))
            } else {
                Err(ProxyError::ResponseStoreError(
                    "response body isn't set".to_string(),
                ))
            }
        } else {
            Err(ProxyError::ResponseStoreError(
                "response header isn't set".to_string(),
            ))
        }
    }

    async fn set_current_requeste(&self, current_request: sync::Arc<sync::Mutex<CurrentExchange>>) {
        let mut current_request = current_request.lock().unwrap();
        let (headers, body) = self.get_request().unwrap();
        current_request.headers = headers;
        current_request.body = body;
    }
}

async fn reqwest2hyper(
    reqw_response: reqwest::Response,
) -> Result<hyper::Response<Body>, ProxyError> {
    let reqw_headers = reqw_response.headers().to_owned();
    let reqw_body = reqw_response.bytes().await?;

    let hyper_body = hyper::Body::from(reqw_body);

    let mut hyper_response = hyper::Response::builder();
    for (name, value) in reqw_headers {
        if let Some(name) = name {
            hyper_response = hyper_response.header(name, value);
        }
    }
    let hyper_response = hyper_response.body(hyper_body)?;

    Ok(hyper_response)
}

async fn hyper2reqwest(
    hyper_request: hyper::Request<Body>,
) -> Result<reqwest::Request, ProxyError> {
    let (parts, body) = hyper_request.into_parts();
    let body = hyper::body::to_bytes(body).await?;

    let reqw_body = reqwest::Body::from(body);
    let reqw_headers = parts.headers;
    let reqw_url = match reqwest::Url::parse(parts.uri.to_string().as_str()) {
        Ok(url) => Ok(url),
        Err(error) => Err(ProxyError::UriParseError(error.to_string())),
    };
    let reqw_url = reqw_url?;
    let reqw_method = match parts.method {
        hyper::http::Method::GET => Ok(reqwest::Method::GET),
        hyper::http::Method::POST => Ok(reqwest::Method::PUT),
        hyper::http::Method::DELETE => Ok(reqwest::Method::DELETE),
        hyper::http::Method::HEAD => Ok(reqwest::Method::HEAD),
        hyper::http::Method::OPTIONS => Ok(reqwest::Method::OPTIONS),
        hyper::http::Method::CONNECT => Ok(reqwest::Method::CONNECT),
        hyper::http::Method::PATCH => Ok(reqwest::Method::PATCH),
        hyper::http::Method::TRACE => Ok(reqwest::Method::TRACE),
        _ => Err(ProxyError::InvalidMethodError),
    };
    let reqw_method = reqw_method?;

    let reqw_client = reqwest::Client::new();
    let reqw_request_builder = reqw_client
        .request(reqw_method, reqw_url)
        .headers(reqw_headers)
        .body(reqw_body);

    let reqw_request = reqw_request_builder.build()?;
    Ok(reqw_request)
}
