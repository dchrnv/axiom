use crate::state::AppData;
use egui::Ui;

pub fn show(ui: &mut Ui, data: &AppData) {
    ui.heading("Domains");
    ui.separator();

    if data.domains.is_empty() {
        ui.label(egui::RichText::new("no data yet").italics());
        return;
    }

    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("domains_grid")
            .num_columns(4)
            .striped(true)
            .show(ui, |ui| {
                ui.strong("id");
                ui.strong("name");
                ui.strong("tokens");
                ui.strong("connections");
                ui.end_row();

                for d in &data.domains {
                    ui.label(d.domain_id.to_string());
                    ui.label(&d.name);
                    ui.label(d.token_count.to_string());
                    ui.label(d.connection_count.to_string());
                    ui.end_row();
                }
            });
    });
}
