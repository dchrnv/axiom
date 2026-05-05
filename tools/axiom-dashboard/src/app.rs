use crate::panels::{input, space_view, status, traces};
use crate::state::AppData;
use eframe::egui;
use std::sync::{Arc, Mutex};

pub struct DashboardApp {
    data: Arc<Mutex<AppData>>,
    cmd_tx: std::sync::mpsc::Sender<String>,
    input_text: String,
}

impl DashboardApp {
    pub fn new(data: Arc<Mutex<AppData>>, cmd_tx: std::sync::mpsc::Sender<String>) -> Self {
        Self {
            data,
            cmd_tx,
            input_text: String::new(),
        }
    }
}

impl eframe::App for DashboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Постоянное перерисовывание чтобы UI обновлялся при новых WS-сообщениях
        ctx.request_repaint_after(std::time::Duration::from_millis(100));

        let data = self.data.lock().unwrap();

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("AXIOM Dashboard");
                ui.separator();
                let status = if data.connected {
                    "🟢 connected"
                } else {
                    "🔴 disconnected"
                };
                ui.label(status);
            });
        });

        egui::SidePanel::left("left_panel")
            .min_width(200.0)
            .show(ctx, |ui| {
                status::show(ui, &data);
                ui.add_space(12.0);
                traces::show(ui, &data);
            });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            drop(data); // освобождаем лок до передачи в input::show
            let data = self.data.lock().unwrap();
            input::show(ui, &data, &mut self.input_text, &self.cmd_tx);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let data = self.data.lock().unwrap();
            space_view::show(ui, &data);
        });
    }
}
