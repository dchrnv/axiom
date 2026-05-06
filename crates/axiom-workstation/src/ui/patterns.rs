use std::collections::VecDeque;

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Color, Element, Length};

use crate::app::{FrameEvent, Message, PatternsState};

const SEMANTIC_LAYERS: &[(&str, &str)] = &[
    ("L1", "physical"),
    ("L2", "sensory"),
    ("L3", "motor"),
    ("L4", "emotional"),
    ("L5", "cognitive"),
    ("L6", "social"),
    ("L7", "temporal"),
    ("L8", "abstract"),
];

pub fn patterns_view<'a>(state: &'a PatternsState) -> Element<'a, Message> {
    column![
        active_layers_panel(state),
        recent_frames_panel(state),
    ]
    .into()
}

// ── Active layers ──────────────────────────────────────────────────────────

fn active_layers_panel<'a>(state: &'a PatternsState) -> Element<'a, Message> {
    let current: [u8; 8] =
        std::array::from_fn(|i| state.layer_history[i].front().copied().unwrap_or(0));

    let rows: Vec<Element<Message>> = SEMANTIC_LAYERS
        .iter()
        .enumerate()
        .map(|(i, (code, name))| {
            let val = current[i];
            let spark = sparkline(&state.layer_history[i]);
            let level = level_label(val);
            let level_color = level_color(val);

            row![
                text(format!("{} {:<9}", code, name)).size(12).width(120),
                text(spark).size(12).font(iced::Font::MONOSPACE),
                text(level).size(11).color(level_color),
            ]
            .spacing(10)
            .into()
        })
        .collect();

    container(
        column![
            text("Active layers")
                .size(13)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
            column(rows).spacing(3),
        ]
        .spacing(8),
    )
    .padding(12)
    .into()
}

fn sparkline(history: &VecDeque<u8>) -> String {
    const CHARS: &[char] = &[' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    // Collect up to 20 values oldest-first for display
    let vals: Vec<u8> = history.iter().rev().take(20).copied().collect();
    if vals.is_empty() {
        return " ".repeat(20);
    }
    let mut out = String::with_capacity(20);
    for _ in 0..(20 - vals.len()) {
        out.push(' ');
    }
    for &v in &vals {
        let idx = ((v as usize * 8) / 256).min(8);
        out.push(CHARS[idx]);
    }
    out
}

fn level_label(val: u8) -> &'static str {
    match val {
        0..=10 => "silent",
        11..=60 => "low",
        61..=150 => "medium",
        151..=220 => "high",
        _ => "highest",
    }
}

fn level_color(val: u8) -> Color {
    match val {
        0..=10 => Color::from_rgb(0.35, 0.35, 0.35),
        11..=60 => Color::from_rgb(0.5, 0.5, 0.5),
        61..=150 => Color::from_rgb(0.4, 0.65, 0.9),
        151..=220 => Color::from_rgb(0.3, 0.75, 0.4),
        _ => Color::from_rgb(1.0, 0.85, 0.3),
    }
}

// ── Recent frames ──────────────────────────────────────────────────────────

const FRAMES_PAGE: usize = 20;

fn recent_frames_panel<'a>(state: &'a PatternsState) -> Element<'a, Message> {
    let frames = &state.recent_frames;

    if frames.is_empty() {
        return container(
            text("No frame events yet.")
                .size(13)
                .color(Color::from_rgb(0.45, 0.45, 0.45)),
        )
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into();
    }

    let visible = if state.show_all_frames {
        frames.len()
    } else {
        frames.len().min(FRAMES_PAGE)
    };
    let mut cards: Vec<Element<Message>> =
        frames.iter().take(visible).map(frame_card).collect();

    if !state.show_all_frames && frames.len() > FRAMES_PAGE {
        cards.push(
            button(
                text(format!("Show more… ({} remaining)", frames.len() - FRAMES_PAGE)).size(12),
            )
            .on_press(Message::PatternsShowMore)
            .style(button::secondary)
            .into(),
        );
    }

    container(
        column![
            text("Recent frames")
                .size(13)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
            scrollable(column(cards).spacing(2)).height(Length::Fill),
        ]
        .spacing(8),
    )
    .padding([0u16, 12u16])
    .height(Length::Fill)
    .into()
}

fn frame_card<'a>(ev: &'a FrameEvent) -> Element<'a, Message> {
    match ev {
        FrameEvent::Crystallized {
            anchor_id,
            layers_present,
            participant_count,
            timestamp_secs,
        } => {
            let layers_str = format_layers(*layers_present);
            column![
                row![
                    text("●").size(13).color(Color::from_rgb(0.3, 0.75, 0.4)),
                    text(format!(
                        " Frame #{anchor_id}  syntactic  {participant_count} participants"
                    ))
                    .size(13),
                ],
                text(format!(
                    "  layers: {}  {}",
                    layers_str,
                    format_ago(*timestamp_secs)
                ))
                .size(11)
                .color(Color::from_rgb(0.55, 0.55, 0.55)),
            ]
            .spacing(1)
            .padding([3u16, 8u16])
            .into()
        }
        FrameEvent::Reactivated {
            anchor_id,
            new_temperature,
            timestamp_secs,
        } => column![
            row![
                text("↻").size(13).color(Color::from_rgb(0.6, 0.5, 0.85)),
                text(format!(
                    " Frame #{anchor_id}  reactivated  temp→{new_temperature}"
                ))
                .size(13),
            ],
            text(format!("  {}", format_ago(*timestamp_secs)))
                .size(11)
                .color(Color::from_rgb(0.55, 0.55, 0.55)),
        ]
        .spacing(1)
        .padding([3u16, 8u16])
        .into(),
        FrameEvent::Vetoed {
            reason,
            timestamp_secs,
        } => column![
            row![
                text("⊗").size(13).color(Color::from_rgb(0.85, 0.3, 0.3)),
                text(" Frame candidate vetoed by GUARDIAN").size(13),
            ],
            text(format!(
                "  reason: \"{}\"  {}",
                reason,
                format_ago(*timestamp_secs)
            ))
            .size(11)
            .color(Color::from_rgb(0.55, 0.55, 0.55)),
        ]
        .spacing(1)
        .padding([3u16, 8u16])
        .into(),
        FrameEvent::Promoted {
            source_anchor_id,
            sutra_anchor_id,
            timestamp_secs,
        } => column![
            row![
                text("↑").size(13).color(Color::from_rgb(0.9, 0.7, 0.2)),
                text(format!(
                    " Frame #{source_anchor_id}  promoted to SUTRA  #{sutra_anchor_id}"
                ))
                .size(13),
            ],
            text(format!("  {}", format_ago(*timestamp_secs)))
                .size(11)
                .color(Color::from_rgb(0.55, 0.55, 0.55)),
        ]
        .spacing(1)
        .padding([3u16, 8u16])
        .into(),
    }
}

fn format_layers(mask: u8) -> String {
    let mut parts = Vec::new();
    for i in 0..8u8 {
        if mask & (1 << i) != 0 {
            parts.push(format!("S{}", i + 1));
        }
    }
    if parts.is_empty() {
        "—".to_string()
    } else {
        parts.join(", ")
    }
}

fn format_ago(timestamp_secs: u64) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let diff = now.saturating_sub(timestamp_secs);
    if diff < 10 {
        "just now".to_string()
    } else if diff < 60 {
        format!("{diff}s ago")
    } else if diff < 3600 {
        format!("{}m ago", diff / 60)
    } else {
        format!("{}h ago", diff / 3600)
    }
}
