// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// axiom-tray — системный трей для Axiom Workstation V2.
//
// StatusNotifierItem (ksni) → GNOME appindicatorsupport extension.
//
// Показывает: статус axiom-node, текущий тик, DREAM-фаза.
// Действия: открыть SPA в браузере, запустить / остановить axiom-node.
//
// Конфигурация через env:
//   AXIOM_PORT=8080          HTTP-порт axiom-node (default: 8080)
//   AXIOM_BIN=<path>         путь к бинарнику axiom-node
//                            (default: target/release/axiom-node)

use ksni::menu::StandardItem;
use ksni::{MenuItem, Tray, TrayService};
use std::process::Child;
use std::thread;
use std::time::Duration;

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_BIN: &str = "target/release/axiom-node";
const POLL_SECS: u64 = 2;
const HTTP_TIMEOUT_SECS: u64 = 1;

// ── Состояние движка ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum DreamState {
    Wake,
    FallingAsleep,
    Dreaming,
    Waking,
}

impl DreamState {
    fn label(&self) -> &'static str {
        match self {
            Self::Wake          => "wake",
            Self::FallingAsleep => "falling asleep",
            Self::Dreaming      => "dreaming ✦",
            Self::Waking        => "waking",
        }
    }
}

// ── Tray ──────────────────────────────────────────────────────────────────────

struct AxiomTray {
    online: bool,
    tick: u64,
    dream: DreamState,
    child: Option<Child>,
    port: u16,
    bin_path: String,
}

impl AxiomTray {
    fn new(port: u16, bin_path: String) -> Self {
        Self {
            online: false,
            tick: 0,
            dream: DreamState::Wake,
            child: None,
            port,
            bin_path,
        }
    }

    fn status_line(&self) -> String {
        if self.online {
            format!("●  tick {}  ·  {}", self.tick, self.dream.label())
        } else {
            "○  axiom-node offline".to_string()
        }
    }
}

impl Tray for AxiomTray {
    fn id(&self) -> String {
        "axiom-tray".to_string()
    }

    fn title(&self) -> String {
        if self.online {
            format!("Axiom  {}  {}", self.tick, self.dream.label())
        } else {
            "Axiom — offline".to_string()
        }
    }

    fn icon_name(&self) -> String {
        if self.online {
            "media-playback-start".to_string()
        } else {
            "media-playback-stop".to_string()
        }
    }

    fn tool_tip(&self) -> ksni::ToolTip {
        ksni::ToolTip {
            title: self.status_line(),
            ..Default::default()
        }
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        let status_line = self.status_line();
        let online = self.online;
        let has_child = self.child.is_some();
        let port = self.port;

        vec![
            // ── Статус (не кликабельный) ──────────────────────────────────
            StandardItem {
                label: status_line,
                enabled: false,
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,

            // ── Открыть Workstation ───────────────────────────────────────
            StandardItem {
                label: "Open Workstation".to_string(),
                enabled: online,
                activate: Box::new(move |_| {
                    let _ = open::that(format!("http://127.0.0.1:{port}"));
                }),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,

            // ── Start / Stop / External ───────────────────────────────────
            if !online {
                StandardItem {
                    label: "Start axiom-node".to_string(),
                    activate: Box::new(|this: &mut Self| {
                        if this.child.is_none() {
                            match std::process::Command::new(&this.bin_path).spawn() {
                                Ok(child) => this.child = Some(child),
                                Err(e) => eprintln!("axiom-tray: cannot start axiom-node: {e}"),
                            }
                        }
                    }),
                    ..Default::default()
                }
                .into()
            } else if has_child {
                StandardItem {
                    label: "Stop axiom-node".to_string(),
                    activate: Box::new(|this: &mut Self| {
                        if let Some(ref mut child) = this.child {
                            let _ = child.kill();
                            let _ = child.wait();
                        }
                        this.child = None;
                    }),
                    ..Default::default()
                }
                .into()
            } else {
                // запущен снаружи — не можем остановить
                StandardItem {
                    label: "axiom-node (external)".to_string(),
                    enabled: false,
                    ..Default::default()
                }
                .into()
            },
            MenuItem::Separator,

            // ── Quit ──────────────────────────────────────────────────────
            StandardItem {
                label: "Quit".to_string(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
}

// ── Metrics polling ───────────────────────────────────────────────────────────

fn poll(port: u16) -> Option<(u64, DreamState)> {
    let url = format!("http://127.0.0.1:{port}/metrics");
    let body = ureq::get(&url)
        .timeout(Duration::from_secs(HTTP_TIMEOUT_SECS))
        .call()
        .ok()?
        .into_string()
        .ok()?;
    parse_metrics(&body)
}

fn parse_metrics(body: &str) -> Option<(u64, DreamState)> {
    let mut tick = 0u64;
    let mut dream = DreamState::Wake;
    let mut found = false;
    for line in body.lines() {
        if let Some(val) = line.strip_prefix("engine_tick_total ") {
            tick = val.trim().parse().unwrap_or(0);
            found = true;
        }
        if line.starts_with("engine_state{") {
            dream = if line.contains("state=\"dreaming\"") {
                DreamState::Dreaming
            } else if line.contains("state=\"falling_asleep\"") {
                DreamState::FallingAsleep
            } else if line.contains("state=\"waking\"") {
                DreamState::Waking
            } else {
                DreamState::Wake
            };
        }
    }
    found.then_some((tick, dream))
}

// ── main ──────────────────────────────────────────────────────────────────────

fn main() {
    let port = std::env::var("AXIOM_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_PORT);

    let bin_path = std::env::var("AXIOM_BIN").unwrap_or_else(|_| DEFAULT_BIN.to_string());

    let service = TrayService::new(AxiomTray::new(port, bin_path));
    let handle = service.handle();
    service.spawn();

    thread::spawn(move || loop {
        let result = poll(port);
        handle.update(|t| match result {
            Some((tick, dream)) => {
                t.online = true;
                t.tick = tick;
                t.dream = dream;
            }
            None => {
                t.online = false;
            }
        });
        thread::sleep(Duration::from_secs(POLL_SECS));
    });

    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn metrics_sample(tick: u64, state: &str) -> String {
        format!(
            "# Engine\nengine_tick_total {tick}\nengine_state{{state=\"{state}\"}} 1\n"
        )
    }

    #[test]
    fn test_parse_wake() {
        let (tick, dream) = parse_metrics(&metrics_sample(42, "wake")).unwrap();
        assert_eq!(tick, 42);
        assert_eq!(dream, DreamState::Wake);
    }

    #[test]
    fn test_parse_dreaming() {
        let (tick, dream) = parse_metrics(&metrics_sample(1000, "dreaming")).unwrap();
        assert_eq!(tick, 1000);
        assert_eq!(dream, DreamState::Dreaming);
    }

    #[test]
    fn test_parse_falling_asleep() {
        let (_, dream) = parse_metrics(&metrics_sample(0, "falling_asleep")).unwrap();
        assert_eq!(dream, DreamState::FallingAsleep);
    }

    #[test]
    fn test_parse_waking() {
        let (_, dream) = parse_metrics(&metrics_sample(0, "waking")).unwrap();
        assert_eq!(dream, DreamState::Waking);
    }

    #[test]
    fn test_parse_empty_returns_none() {
        assert!(parse_metrics("# no data").is_none());
    }

    #[test]
    fn test_parse_missing_state_defaults_wake() {
        let body = "engine_tick_total 99\n";
        let (tick, dream) = parse_metrics(body).unwrap();
        assert_eq!(tick, 99);
        assert_eq!(dream, DreamState::Wake);
    }
}
