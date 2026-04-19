// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// axiom-dashboard — реальный времени визуализация AXIOM Engine.
//
// Usage: axiom-dashboard [--server ws://host:port/ws]
//
// Требует запущенного axiom-cli --server.

mod app;
mod panels;
mod protocol;
mod state;
mod ws_client;

use std::sync::{Arc, Mutex};
use app::DashboardApp;
use state::AppData;
use ws_client::run_ws_client;

fn main() {
    let server_url = parse_server_url();
    println!("axiom-dashboard: connecting to {server_url}");

    let data   = Arc::new(Mutex::new(AppData::default()));
    let (tx, rx) = std::sync::mpsc::channel::<String>();

    // WS-клиент в отдельном потоке
    let data_ws = Arc::clone(&data);
    let url_clone = server_url.clone();
    std::thread::spawn(move || {
        run_ws_client(&url_clone, data_ws, rx);
    });

    // eframe GUI на главном потоке
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("AXIOM Dashboard")
            .with_inner_size([1100.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "AXIOM Dashboard",
        options,
        Box::new(|_cc| Ok(Box::new(DashboardApp::new(data, tx)))),
    ).unwrap();
}

fn parse_server_url() -> String {
    let args: Vec<String> = std::env::args().collect();
    args.windows(2)
        .find(|w| w[0] == "--server")
        .map(|w| w[1].clone())
        .unwrap_or_else(|| "ws://127.0.0.1:8765/ws".to_string())
}
