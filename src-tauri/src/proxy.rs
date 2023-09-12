use std::{
    convert::Infallible,
    io::Read,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tauri::{AppHandle, Manager};
use tokio::sync::{broadcast, mpsc};

use hyper::Server;
use thiserror::Error;

use crate::types::{
    BodyString, HeadersJson, MethodString, Request, Response, StatusNumber, UrlString,
};

pub async fn run_proxy_server(pilot_state: Arc<Mutex<bool>>, app_handle: AppHandle) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let make_service = hyper::service::make_service_fn(move |_conn| {
        let app_handle = app_handle.clone();
        let pilot_state = pilot_state.clone();
        async move {
            Ok::<_, Infallible>(hyper::service::service_fn(
                move |request: hyper::Request<hyper::Body>| {
                    let app_handle = app_handle.clone();
                    let pilot_state = pilot_state.clone();
                    async move {
                        Ok::<_, Infallible>(match handle(request, pilot_state, app_handle).await {
                            Ok(response) => response,
                            Err(error_response) => error_response,
                        })
                    }
                },
            ))
        }
    });
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        println!("error: {}", e);
    }
}

// ** ####################################################################################################
// ** Suitable Error struct
// ** ####################################################################################################
#[derive(Error, Debug)]
enum ProxyError {
    #[error("Hyper error: {0}")]
    Hyper(#[from] hyper::Error),
    #[error("Hyper http error: {0}")]
    HyperHttpError(#[from] hyper::http::Error),
    #[error("failed to parse hyper's uri as reqwest's url")]
    UriParseError(String),
    #[error("invalid http method is given")]
    InvalidMethodError,
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Request information is insufficient: {0}")]
    RequestInformationInsufficient(String),
    #[error("Response information is insufficient: {0}")]
    ResponseInformationInsufficient(String),
    #[error("serde json error")]
    SerdeJson(#[from] serde_json::Error),
    #[error("std io error")]
    Io(#[from] std::io::Error),
    #[error("convert response error")]
    ConvertResponse(String),
    #[error("Communication using TLS is not yet supported")]
    UnavailableHttpsError,
}

// ** ####################################################################################################
// ** Handling proxy block
// ** ####################################################################################################
async fn handle(
    request: hyper::Request<hyper::Body>,
    shared_pilot_state: Arc<Mutex<bool>>,
    app_handle: AppHandle,
) -> Result<hyper::Response<hyper::Body>, hyper::Response<hyper::Body>> {
    // * convert hyper request to reqwest request and make exchange from them.
    // convert hyper request to reqwest request
    let reqw_request = hyper2reqwest(request).await;
    if let Err(error) = reqw_request {
        return Err(generate_error_response(error));
    }
    let reqw_request = reqw_request.unwrap();
    // make exchange from reqwest request
    let mut exchange = VolatileExchange::new(reqw_request, shared_pilot_state, app_handle);

    // *
    // * REQUEST
    // *

    // * get info of request, set them to property, and send them to frontend.
    // get info of request and set them to property.
    let result = exchange.fetch_request().await;
    if let Err(error) = result {
        return Err(generate_error_response(error));
    }
    // send info of request to frontend.
    // ** check pilot_state and set it to exchange property
    exchange.check_pilot_state().await;
    let request_for_front = exchange.get_request_for_front();
    exchange
        .app_handle
        .emit_all("proxy_request", &request_for_front)
        .unwrap();

    if exchange.pilot_state {
        exchange.wait_for_request_modified().await;
    }

    // *
    // * RESPONSE
    // *

    // * communicate with remote host, get response from it, set info of response to the property, and send them to the frontend.
    // communicate with remote host, and set response from it to property
    let result = exchange.fetch_response().await;
    if let Err(error) = result {
        return Err(generate_error_response(error));
    }
    // send response to the frontend.
    let response_for_front = exchange.get_response_for_front();
    exchange
        .app_handle
        .emit_all("proxy_response", &response_for_front)
        .unwrap();

    if let Some(reqw_response) = exchange.response {
        let response = reqwest2hyper(reqw_response).await;
        if let Err(error) = response {
            Err(generate_error_response(error))
        } else {
            Ok(response.unwrap())
        }
    } else {
        Err(generate_error_response(ProxyError::ConvertResponse(
            "response isn't set".to_string(),
        )))
    }
}

struct VolatileExchange {
    request: reqwest::Request,
    response: Option<reqwest::Response>,
    request_body: Option<String>,
    response_body: Option<String>,
    request_headers: Option<String>,
    response_headers: Option<String>,
    request_url: Option<String>,
    response_url: Option<String>,
    request_method: Option<String>,
    response_status: Option<u16>,
    pilot_state: bool,
    shared_pilot_state: Arc<Mutex<bool>>,
    app_handle: AppHandle,
}

impl VolatileExchange {
    fn new(
        request: reqwest::Request,
        shared_pilot_state: Arc<Mutex<bool>>,
        app_handle: AppHandle,
    ) -> Self {
        VolatileExchange {
            request,
            response: None,
            request_headers: None,
            response_headers: None,
            request_body: None,
            response_body: None,
            request_url: None,
            response_url: None,
            request_method: None,
            response_status: None,
            pilot_state: false,
            shared_pilot_state,
            app_handle,
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

        // ** url
        let url = self.request.url();
        self.request_url = Some(url.to_string());

        // ** method
        let method = self.request.method();
        self.request_method = Some(method.to_string());

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

        // ** url
        let url = response.url();
        self.response_url = Some(url.to_string());

        // ** status
        let status = response.status();
        self.response_status = Some(status.as_u16());

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

    fn get_request(
        &self,
    ) -> Result<(HeadersJson, BodyString, UrlString, MethodString), ProxyError> {
        if let Some(header) = self.request_headers.clone() {
            if let Some(body) = self.request_body.clone() {
                if let Some(url) = self.request_url.clone() {
                    if let Some(method) = self.request_method.clone() {
                        Ok((header, body, url, method))
                    } else {
                        Err(ProxyError::RequestInformationInsufficient(
                            "request method isn't set for volatile exchange".to_string(),
                        ))
                    }
                } else {
                    Err(ProxyError::RequestInformationInsufficient(
                        "request url isn't set for volatile exchange".to_string(),
                    ))
                }
            } else {
                Err(ProxyError::RequestInformationInsufficient(
                    "request body isn't set for volatile exchange".to_string(),
                ))
            }
        } else {
            Err(ProxyError::RequestInformationInsufficient(
                "request header isn't set for volatile exchange".to_string(),
            ))
        }
    }

    fn get_response(
        &self,
    ) -> Result<(HeadersJson, BodyString, UrlString, StatusNumber), ProxyError> {
        if let Some(header) = self.response_headers.clone() {
            if let Some(body) = self.response_body.clone() {
                if let Some(url) = self.response_url.clone() {
                    if let Some(status) = self.response_status {
                        Ok((header, body, url, status))
                    } else {
                        Err(ProxyError::ResponseInformationInsufficient(
                            "response status isn't set".to_string(),
                        ))
                    }
                } else {
                    Err(ProxyError::ResponseInformationInsufficient(
                        "response url isn't set".to_string(),
                    ))
                }
            } else {
                Err(ProxyError::ResponseInformationInsufficient(
                    "response body isn't set".to_string(),
                ))
            }
        } else {
            Err(ProxyError::ResponseInformationInsufficient(
                "response header isn't set".to_string(),
            ))
        }
    }

    fn get_request_for_front(&self) -> String {
        let (headers, body, url, method) = self.get_request().unwrap();
        let request = Request {
            headers,
            body,
            url,
            method,
            piloted: self.pilot_state,
        };
        serde_json::to_string(&request).unwrap()
    }

    fn get_response_for_front(&self) -> String {
        let (headers, body, url, status) = self.get_response().unwrap();
        let response = Response {
            headers,
            body,
            url,
            status,
            piloted: self.pilot_state,
        };
        serde_json::to_string(&response).unwrap()
    }

    async fn check_pilot_state(&mut self) {
        self.pilot_state = *self.shared_pilot_state.lock().unwrap();
    }

    async fn wait_for_request_modified(&mut self) {
        let app_handle = self.app_handle.clone();
        let (modified_request_sender, mut modified_request_receiver) =
            mpsc::channel::<Request>(200);

        app_handle.listen_global("send-modified-request", move |event| {
            let modified_request_sender = modified_request_sender.clone();
            tokio::spawn(async move {
                let modified_request_str = event.payload().unwrap();
                println!("{}", modified_request_str);
                let modified_request: Request = serde_json::from_str(modified_request_str).unwrap();
                modified_request_sender
                    .send(modified_request)
                    .await
                    .unwrap();
            });
        });

        if let Some(modified_request) = modified_request_receiver.recv().await {
            println!("hello!");
        }
    }
}

fn generate_error_response(error: ProxyError) -> hyper::Response<hyper::Body> {
    hyper::Response::builder()
        .status(500)
        .body(hyper::Body::from(error.to_string()))
        .unwrap()
}

async fn reqwest2hyper(
    reqw_response: reqwest::Response,
) -> Result<hyper::Response<hyper::Body>, ProxyError> {
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
    hyper_request: hyper::Request<hyper::Body>,
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

    if reqw_url.to_string().contains("https") || reqw_url.to_string().contains(":443") {
        return Err(ProxyError::UnavailableHttpsError);
    }

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

    let reqw_version = parts.version;

    let reqw_method = reqw_method?;

    let reqw_client = reqwest::Client::new();
    let reqw_request_builder = reqw_client
        .request(reqw_method, reqw_url)
        .headers(reqw_headers)
        .version(reqw_version)
        .body(reqw_body);

    let reqw_request = reqw_request_builder.build();
    let reqw_request = reqw_request.unwrap();
    Ok(reqw_request)
}
