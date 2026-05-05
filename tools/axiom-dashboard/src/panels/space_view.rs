// Space View — 2D визуализация токенов и доменов.
//
// Если токены загружены через DomainDetail — показывает их как точки:
//   позиция = token.position[0,1], размер ~ mass, цвет = domain_id
//   якоря (is_anchor) = крест поверх точки
//
// Если токенов нет — показывает домены в кольцевом расположении.

use crate::state::AppData;
use egui::Ui;
use egui_plot::{MarkerShape, Plot, PlotPoints, Points};

type DomainBuckets = std::collections::HashMap<u16, (Vec<[f64; 2]>, Vec<[f64; 2]>)>;

/// Фиксированная палитра для 11 доменов (domain_id → цвет)
fn domain_color(domain_id: u16) -> egui::Color32 {
    const PALETTE: [egui::Color32; 11] = [
        egui::Color32::from_rgb(100, 180, 255), // SUTRA    100
        egui::Color32::from_rgb(180, 100, 255), // SHADOW   200
        egui::Color32::from_rgb(100, 255, 180), // CODEX    300
        egui::Color32::from_rgb(255, 200, 100), // MAP      400
        egui::Color32::from_rgb(255, 100, 100), // PROBE    500
        egui::Color32::from_rgb(100, 255, 100), // LOGIC    600
        egui::Color32::from_rgb(255, 255, 100), // DREAM    700
        egui::Color32::from_rgb(200, 100, 180), // ETHICS   800
        egui::Color32::from_rgb(180, 180, 255), // IMPULSE  900
        egui::Color32::from_rgb(255, 180, 100), // DRIVE   1000
        egui::Color32::from_rgb(150, 255, 255), // META    1100
    ];
    let idx = ((domain_id / 100).saturating_sub(1) as usize).min(10);
    PALETTE[idx]
}

pub fn show(ui: &mut Ui, data: &AppData) {
    ui.heading("Space View");
    ui.separator();

    if !data.tokens.is_empty() {
        show_tokens(ui, data);
    } else {
        show_domains(ui, data);
    }
}

fn show_tokens(ui: &mut Ui, data: &AppData) {
    Plot::new("space_view_tokens")
        .data_aspect(1.0)
        .show(ui, |plot_ui| {
            // Группируем по domain_id для разных цветов
            let mut by_domain: DomainBuckets = DomainBuckets::new();

            for (did, tok) in &data.tokens {
                let x = tok.position[0] as f64;
                let y = tok.position[1] as f64;
                let (normal, anchors) = by_domain.entry(*did).or_default();
                if tok.is_anchor {
                    anchors.push([x, y]);
                } else {
                    normal.push([x, y]);
                }
            }

            for (did, (normal, anchors)) in &by_domain {
                let color = domain_color(*did);
                if !normal.is_empty() {
                    let pts = Points::new(PlotPoints::from(normal.clone()))
                        .color(color)
                        .radius(3.0)
                        .shape(MarkerShape::Circle);
                    plot_ui.points(pts);
                }
                if !anchors.is_empty() {
                    let pts = Points::new(PlotPoints::from(anchors.clone()))
                        .color(color)
                        .radius(5.0)
                        .shape(MarkerShape::Cross);
                    plot_ui.points(pts);
                }
            }
        });
}

fn show_domains(ui: &mut Ui, data: &AppData) {
    if data.domains.is_empty() {
        ui.label(egui::RichText::new("no data — waiting for state broadcast").italics());
        return;
    }

    // Домены в кольцевом расположении
    let n = data.domains.len() as f64;
    Plot::new("space_view_domains")
        .data_aspect(1.0)
        .show(ui, |plot_ui| {
            for (i, d) in data.domains.iter().enumerate() {
                let angle = 2.0 * std::f64::consts::PI * i as f64 / n;
                let x = angle.cos() * 10.0;
                let y = angle.sin() * 10.0;
                let size = ((d.token_count as f64 + 1.0).ln() * 3.0 + 2.0) as f32;
                let color = domain_color(d.domain_id);
                let pts = Points::new(PlotPoints::from(vec![[x, y]]))
                    .color(color)
                    .radius(size)
                    .shape(MarkerShape::Circle)
                    .name(&d.name);
                plot_ui.points(pts);
            }
        });

    ui.small("tip: send DomainDetail requests to populate per-token view");
}
