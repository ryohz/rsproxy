// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod exchange;
mod proxy;
use std::sync::{Arc, Mutex};

use proxy::run_proxy_server;
use tauri::Manager;

#[tokio::main]
async fn main() {
    // ** shared state for proxy
    let current_request = Arc::new(Mutex::new(exchange::Exchange {
        headers: "".to_string(),
        body: "".to_string(),
    }));
    let current_response = Arc::new(Mutex::new(exchange::Exchange {
        headers: "".to_string(),
        body: "".to_string(),
    }));
    let pilot_state = Arc::new(Mutex::new(false));

    let current_request_for_proxy_server = current_request.clone();
    let current_response_for_proxy_server = current_response.clone();
    let pilot_state_for_proxy_server = pilot_state.clone();

    tokio::spawn(async move {
        run_proxy_server(
            current_request_for_proxy_server,
            current_response_for_proxy_server,
            pilot_state_for_proxy_server,
        )
        .await;
    });

    let current_request_for_change_detection = Arc::clone(&current_request);
    let current_response_for_change_detection = Arc::clone(&current_response);
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.app_handle();
            tokio::spawn(async move {
                let current_request = current_request_for_change_detection.clone();
                let current_response = current_response_for_change_detection.clone();
                loop {
                    let mut current_request = current_request.lock().unwrap();
                    let mut current_response = current_response.lock().unwrap();

                    if !current_request.headers.is_empty() {
                        app_handle
                            .emit_all("proxy_request", &current_request.headers)
                            .unwrap();
                        println!("[/]request");
                        current_request.headers = "".to_string();
                    }

                    if !current_response.headers.is_empty() {
                        app_handle
                            .emit_all("proxy_response", &current_response.headers)
                            .unwrap();
                        println!("[/]response");
                        current_response.headers = "".to_string();
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
