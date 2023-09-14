use std::{
    convert::Infallible,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tauri::AppHandle;

use hyper::Server;

use crate::http_util::{self, traits::Json};

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
        let rq_front = http_util::request::Request::from_hyper(request).await;
        rq_front.send_to_front(&app_handle).await;
        let m_rq_front = rq_front.wait_for_modification(&app_handle).await;
        m_rq_front.to_hyper().await
    } else {
        let (rq1, rq2) = http_util::request::copy_request(request).await;
        let rq_front = http_util::request::Request::from_hyper(rq1).await;
        rq_front.send_to_front(&app_handle).await;
        rq2
    };

    let client = hyper::Client::new();
    let response = client.request(request).await.unwrap();

    if pilot_state(shared_pilot_state.clone()) {
        let rs_front = http_util::response::Response::from_hyper(response).await;
        rs_front.send_to_front(&app_handle).await;
        let m_rs_front = rs_front.wait_for_modification(&app_handle).await;
        m_rs_front.to_hyper().await
    } else {
        let (rs1, rs2) = http_util::response::copy_response(response).await;
        let rs_front = http_util::response::Response::from_hyper(rs1).await;
        rs_front.send_to_front(&app_handle).await;
        rs2
    }
}

fn pilot_state(shared_pilot_state: Arc<Mutex<bool>>) -> bool {
    *shared_pilot_state.lock().unwrap()
}
