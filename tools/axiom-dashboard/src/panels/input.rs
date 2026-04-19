use egui::Ui;
use crate::state::AppData;

pub fn show(
    ui:     &mut Ui,
    data:   &AppData,
    input:  &mut String,
    cmd_tx: &std::sync::mpsc::Sender<String>,
) {
    ui.heading("Input");
    ui.separator();

    // Текстовый ввод
    let resp = ui.add(
        egui::TextEdit::singleline(input)
            .hint_text("inject text or :command")
            .desired_width(f32::INFINITY),
    );

    let submit = resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

    ui.horizontal(|ui| {
        if ui.button("Send").clicked() || submit {
            send_input(input.trim(), cmd_tx);
            input.clear();
        }
        if ui.button(":status").clicked() {
            send_read(":status", cmd_tx);
        }
        if ui.button(":domains").clicked() {
            send_read(":domains", cmd_tx);
        }
        if ui.button(":traces").clicked() {
            send_read(":traces", cmd_tx);
        }
    });

    // Вывод последней команды
    if !data.last_output.is_empty() {
        ui.separator();
        ui.label("Last output:");
        egui::ScrollArea::vertical()
            .max_height(120.0)
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut data.last_output.as_str())
                        .desired_width(f32::INFINITY)
                        .font(egui::TextStyle::Monospace),
                );
            });
    }
}

fn send_input(text: &str, tx: &std::sync::mpsc::Sender<String>) {
    if text.is_empty() { return; }
    let json = if text.starts_with(':') {
        let cmd = text.split_whitespace().next().unwrap_or(text);
        let is_mutate = matches!(cmd,
            ":save"|":load"|":autosave"|":tick"|":export"|":import"|":quit"|":q"
        );
        if is_mutate {
            serde_json::json!({ "type": "mutate_command", "cmd": text }).to_string()
        } else {
            serde_json::json!({ "type": "read_command", "cmd": text }).to_string()
        }
    } else {
        serde_json::json!({ "type": "inject", "text": text }).to_string()
    };
    let _ = tx.send(json);
}

fn send_read(cmd: &str, tx: &std::sync::mpsc::Sender<String>) {
    let json = serde_json::json!({ "type": "read_command", "cmd": cmd }).to_string();
    let _ = tx.send(json);
}
