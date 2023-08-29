use std::{
    convert::Infallible,
    io::Read,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use hyper::Server;
use thiserror::Error;

use crate::exchange::Exchange;

pub async fn run_proxy_server(
    current_request: Arc<Mutex<Exchange>>,
    current_response: Arc<Mutex<Exchange>>,
    pilot_state: Arc<Mutex<bool>>,
) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let make_service = hyper::service::make_service_fn(move |_conn| {
        let current_request = current_request.clone();
        let current_response = current_response.clone();
        let pilot_state = pilot_state.clone();
        async move {
            Ok::<_, Infallible>(hyper::service::service_fn(
                move |request: hyper::Request<hyper::Body>| {
                    let current_request = current_request.clone();
                    let current_response = current_response.clone();
                    let pilot_state = pilot_state.clone();
                    async move {
                        Ok::<_, Infallible>(
                            match handle(request, current_request, current_response, pilot_state)
                                .await
                            {
                                Ok(response) => response,
                                Err(error_response) => error_response,
                            },
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
    #[error("request store error")]
    RequestStore(String),
    #[error("serde json error")]
    SerdeJson(#[from] serde_json::Error),
    #[error("std io error")]
    Io(#[from] std::io::Error),
    #[error("response store error")]
    ResponseStore(String),
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
    current_request: Arc<Mutex<Exchange>>,
    current_response: Arc<Mutex<Exchange>>,
    pilot_state: Arc<Mutex<bool>>,
) -> Result<hyper::Response<hyper::Body>, hyper::Response<hyper::Body>> {
    // ** convert hyper request to reqwest request
    let reqw_request = hyper2reqwest(request).await;
    if let Err(error) = reqw_request {
        return Err(generate_error_response(error));
    }
    let reqw_request = reqw_request.unwrap();

    let mut exchange = VolatileExchange::new(reqw_request, false);

    // ** get request headers and body, and set them to exchange's property
    let result = exchange.fetch_request().await;
    if let Err(error) = result {
        return Err(generate_error_response(error));
    }
    exchange.set_current_requeste(current_request).await;

    // ** communicate with remote host, get response from it, and set headers and body to exchange's property
    let result = exchange.fetch_response().await;
    if let Err(error) = result {
        return Err(generate_error_response(error));
    }
    exchange.set_current_response(current_response).await;

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
}

impl VolatileExchange {
    fn new(request: reqwest::Request, pilot_flag: bool) -> Self {
        VolatileExchange {
            request,
            response: None,
            request_headers: None,
            response_headers: None,
            request_body: None,
            response_body: None,
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
                Err(ProxyError::RequestStore(
                    "request body isn't set".to_string(),
                ))
            }
        } else {
            Err(ProxyError::RequestStore(
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
                Err(ProxyError::ResponseStore(
                    "response body isn't set".to_string(),
                ))
            }
        } else {
            Err(ProxyError::ResponseStore(
                "response header isn't set".to_string(),
            ))
        }
    }

    async fn set_current_requeste(&self, current_request: Arc<Mutex<Exchange>>) {
        let mut current_request = current_request.lock().unwrap();
        let (headers, body) = self.get_request().unwrap();
        current_request.headers = headers;
        current_request.body = body;
    }

    async fn set_current_response(&self, current_response: Arc<Mutex<Exchange>>) {
        let mut current_response = current_response.lock().unwrap();
        let (headers, body) = self.get_response().unwrap();
        current_response.headers = headers;
        current_response.body = body;
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
