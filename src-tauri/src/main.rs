// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod proxy;
mod types;
use std::sync::{Arc, Mutex};

use proxy::run_proxy_server;
use tauri::Manager;
use tokio::sync::mpsc;
use types::{Request, Response};

#[tokio::main]
async fn main() {
    // ** shared state for proxy
    // let current_request = Arc::new(Mutex::new(Request {
    //     headers: "".to_string(),
    //     body: "".to_string(),
    //     url: "".to_string(),
    //     method: "".to_string(),
    // }));
    let (request_tx, mut request_rx) = mpsc::channel::<Request>(200);
    let (response_tx, mut response_rx) = mpsc::channel::<Response>(200);
    let (pilot_state_tx, mut pilot_state_rx) = mpsc::channel::<bool>(10);

    let proxy_request_tx = request_tx.clone();
    let proxy_response_tx = response_tx.clone();

    tokio::spawn(async move {
        run_proxy_server(proxy_request_tx, proxy_response_tx).await;
    });

    tauri::Builder::default()
        .setup(|app| {
            let request_app_handle = app.app_handle();
            tokio::spawn(async move {
                while let Some(request) = request_rx.recv().await {
                    println!("hello");
                    let json = serde_json::to_string(&request).unwrap();
                    request_app_handle.emit_all("proxy_request", json).unwrap();
                }
            });

            let response_app_handle = app.app_handle();
            tokio::spawn(async move {
                while let Some(response) = response_rx.recv().await {
                    let json = serde_json::to_string(&response).unwrap();
                    response_app_handle
                        .emit_all("proxy_response", json)
                        .unwrap();
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
