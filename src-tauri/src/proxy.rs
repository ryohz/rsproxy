use std::{
    convert::Infallible,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tauri::AppHandle;

use hyper::Server;

use crate::http_util::{self, traits::HeaderMapMethods};

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
                    async move { Ok::<_, Infallible>(handle(request, pilot_state, app_handle).await) }
                },
            ))
        }
    });
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        println!("error: {}", e);
    }
}

async fn handle(
    request: hyper::Request<hyper::Body>,
    shared_pilot_state: Arc<Mutex<bool>>,
    app_handle: AppHandle,
) -> hyper::Response<hyper::Body> {
    request.headers().check_encoding().unwrap();

    let request = if pilot_state(shared_pilot_state.clone()) {
        let rq_front = match http_util::request::Request::from_hyper(request).await {
            Ok(rq) => rq,
            Err(e) => {
                panic!("proxy error: {}", e);
            }
        };
        if let Err(e) = rq_front.send_to_front(&app_handle).await {
            panic!("proxy error: {}", e);
        }
        let m_rq_front = match rq_front.wait_for_modification(&app_handle).await {
            Ok(rq) => rq,
            Err(e) => {
                panic!("proxy error >>> {}", e);
            }
        };
        match m_rq_front.to_hyper().await {
            Ok(rq) => rq,
            Err(e) => {
                panic!("proxy error >>> {}", e);
            }
        }
    } else {
        let (rq1, rq2) = match http_util::request::copy_request(request).await {
            Ok(t) => t,
            Err(e) => {
                panic!("proxy error >>> {}", e);
            }
        };
        let rq_front = match http_util::request::Request::from_hyper(rq1).await {
            Ok(rq) => rq,
            Err(e) => {
                panic!("proxy error >>> {}", e);
            }
        };
        if let Err(e) = rq_front.send_to_front(&app_handle).await {
            panic!("proxy error >>> {}", e);
        }
        rq2
    };

    let client = hyper::Client::new();
    let response = match client.request(request).await {
        Ok(rs) => rs,
        Err(e) => {
            panic!("proxy error >>> {}", e);
        }
    };

    if pilot_state(shared_pilot_state.clone()) {
        let rs_front = match http_util::response::Response::from_hyper(response).await {
            Ok(rs) => rs,
            Err(e) => {
                panic!("proxy error >>> {}", e);
            }
        };
        if let Err(e) = rs_front.send_to_front(&app_handle).await {
            panic!("proxy error >>> {}", e);
        }
        let m_rs_front = match rs_front.wait_for_modification(&app_handle).await {
            Ok(rs) => rs,
            Err(e) => {
                panic!("proxy error >>> {}", e);
            }
        };
        match m_rs_front.to_hyper().await {
            Ok(rs) => rs,
            Err(e) => {
                panic!("proxy error >>> {}", e)
            }
        }
    } else {
        let (rs1, rs2) = match http_util::response::copy_response(response).await {
            Ok(t) => t,
            Err(e) => {
                panic!("proxy error >>> {}", e)
            }
        };
        let rs_front = match http_util::response::Response::from_hyper(rs1).await {
            Ok(rs) => rs,
            Err(e) => {
                panic!("proxy error >>> {}", e)
            }
        };
        if let Err(e) = rs_front.send_to_front(&app_handle).await {
            panic!("proxy error >>> {}", e)
        }
        rs2
    }
}

fn pilot_state(shared_pilot_state: Arc<Mutex<bool>>) -> bool {
    *shared_pilot_state.lock().unwrap()
}
