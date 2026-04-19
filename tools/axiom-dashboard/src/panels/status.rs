use egui::Ui;
use crate::state::AppData;

pub fn show(ui: &mut Ui, data: &AppData) {
    ui.heading("Status");
    ui.separator();

    let conn_label = if data.connected {
        egui::RichText::new("● connected").color(egui::Color32::GREEN)
    } else {
        egui::RichText::new("● disconnected").color(egui::Color32::RED)
    };
    ui.label(conn_label);

    if let Some(ref err) = data.last_error {
        ui.colored_label(egui::Color32::YELLOW, format!("⚠ {err}"));
    }

    ui.separator();
    egui::Grid::new("status_grid").num_columns(2).show(ui, |ui| {
        ui.label("tick_count");  ui.label(data.tick_count.to_string()); ui.end_row();
        ui.label("traces");      ui.label(data.traces.to_string());     ui.end_row();
        ui.label("tension");     ui.label(data.tension.to_string());    ui.end_row();
        ui.label("last_matched");ui.label(data.last_matched.to_string()); ui.end_row();
        ui.label("domains");     ui.label(data.domains.len().to_string()); ui.end_row();
        ui.label("tokens");      ui.label(data.tokens.len().to_string()); ui.end_row();
    });
}
