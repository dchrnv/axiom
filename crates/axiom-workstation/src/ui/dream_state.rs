use iced::widget::{button, column, container, horizontal_space, row, scrollable, text};
use iced::{Color, Element, Length};

use axiom_protocol::{events::EngineState, snapshot::{DreamReport, SystemSnapshot}};

use crate::app::{DreamWindowState, Message};

pub fn dream_state_view<'a>(
    state: &'a DreamWindowState,
    snapshot: &'a Option<SystemSnapshot>,
) -> Element<'a, Message> {
    let top = row![
        container(current_state_panel(state, snapshot)).width(250),
        fatigue_panel(snapshot),
    ]
    .height(200);

    column![
        top,
        recent_dreams_panel(&state.recent_dreams),
    ]
    .into()
}

// ── Current state ──────────────────────────────────────────────────────────

fn current_state_panel<'a>(
    state: &'a DreamWindowState,
    snapshot: &'a Option<SystemSnapshot>,
) -> Element<'a, Message> {
    let (state_label, state_color, since_ticks) = match snapshot {
        Some(snap) => {
            let since = snap.current_tick
                .saturating_sub(snap.dream_phase_stats.last_transition_tick);
            let (label, color) = engine_state_display(snap.engine_state);
            (label, color, since)
        }
        None => ("—", Color::from_rgb(0.45, 0.45, 0.45), 0u64),
    };

    let engine_state = snapshot.as_ref().map(|s| s.engine_state);

    let action_widget: Element<Message> = if state.confirm_force_sleep {
        // Confirmation inline
        column![
            text("Force engine to sleep?").size(12),
            row![
                button(text("Cancel").size(12))
                    .on_press(Message::ForceSleepCancel)
                    .style(button::secondary),
                button(text("Sleep now").size(12))
                    .on_press(Message::ForceSleepConfirm)
                    .style(button::danger),
            ]
            .spacing(6),
        ]
        .spacing(6)
        .into()
    } else {
        match engine_state {
            Some(EngineState::Wake) => button(text("Force sleep").size(12))
                .on_press(Message::ForceSleepRequest)
                .style(button::secondary)
                .into(),
            Some(EngineState::Dreaming) => button(text("Wake up").size(12))
                .on_press(Message::ForceWakeRequest)
                .style(button::secondary)
                .into(),
            _ => text("transitioning...").size(11)
                .color(Color::from_rgb(0.5, 0.5, 0.5))
                .into(),
        }
    };

    container(
        column![
            text(state_label).size(22).color(state_color),
            text(format!("{} ticks since transition", since_ticks))
                .size(11).color(Color::from_rgb(0.5, 0.5, 0.5)),
            action_widget,
        ]
        .spacing(10),
    )
    .padding(16)
    .into()
}

fn engine_state_display(state: EngineState) -> (&'static str, Color) {
    match state {
        EngineState::Wake => ("WAKE", Color::from_rgb(0.4, 0.65, 0.9)),
        EngineState::FallingAsleep => ("FALLING ASLEEP", Color::from_rgb(0.55, 0.45, 0.75)),
        EngineState::Dreaming => ("DREAMING", Color::from_rgb(0.45, 0.25, 0.7)),
        EngineState::Waking => ("WAKING", Color::from_rgb(0.55, 0.65, 0.8)),
    }
}

// ── Fatigue ────────────────────────────────────────────────────────────────

fn fatigue_panel<'a>(snapshot: &'a Option<SystemSnapshot>) -> Element<'a, Message> {
    let Some(snap) = snapshot else {
        return container(
            text("No data").size(13).color(Color::from_rgb(0.45, 0.45, 0.45)),
        )
        .padding(16)
        .into();
    };

    let f = &snap.fatigue;
    let pct = (f.current * 100.0) as u32;
    let threshold_pct = (f.threshold * 100.0) as u32;
    let spark = fatigue_sparkline(&f.history);

    container(
        column![
            text("Fatigue").size(13).color(Color::from_rgb(0.6, 0.6, 0.6)),
            row![
                text(format!("Total: {}%", pct)).size(13),
                horizontal_space(),
                text(format!("threshold: {}%", threshold_pct))
                    .size(11).color(Color::from_rgb(0.5, 0.5, 0.5)),
            ],
            text(spark).size(12).font(iced::Font::MONOSPACE),
            column![
                text(format!("Token rate: {:.2}/tick", f.token_rate))
                    .size(11).color(Color::from_rgb(0.55, 0.55, 0.55)),
                text(format!("Ticks since dream: {}", f.ticks_since_dream))
                    .size(11).color(Color::from_rgb(0.55, 0.55, 0.55)),
            ]
            .spacing(3),
        ]
        .spacing(8),
    )
    .padding(16)
    .into()
}

fn fatigue_sparkline(history: &[f32]) -> String {
    const CHARS: &[char] = &[' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    if history.is_empty() {
        return " ".repeat(20);
    }
    let start = history.len().saturating_sub(20);
    let mut out = String::with_capacity(20);
    for _ in 0..(20usize.saturating_sub(history.len())) {
        out.push(' ');
    }
    for &v in &history[start..] {
        let idx = ((v.clamp(0.0, 1.0) * 8.0) as usize).min(8);
        out.push(CHARS[idx]);
    }
    out
}

// ── Recent dreams ──────────────────────────────────────────────────────────

fn recent_dreams_panel<'a>(dreams: &'a std::collections::VecDeque<DreamReport>) -> Element<'a, Message> {
    if dreams.is_empty() {
        return container(
            column![
                text("Recent dreams").size(13).color(Color::from_rgb(0.6, 0.6, 0.6)),
                text("No dream cycles recorded yet.")
                    .size(13).color(Color::from_rgb(0.45, 0.45, 0.45)),
            ]
            .spacing(8),
        )
        .padding([0u16, 12u16])
        .into();
    }

    let cards: Vec<Element<Message>> = dreams.iter().map(dream_card).collect();

    container(
        column![
            text("Recent dreams").size(13).color(Color::from_rgb(0.6, 0.6, 0.6)),
            scrollable(column(cards).spacing(4)).height(Length::Fill),
        ]
        .spacing(8),
    )
    .padding([0u16, 12u16])
    .height(Length::Fill)
    .into()
}

fn dream_card<'a>(report: &'a DreamReport) -> Element<'a, Message> {
    let duration_ticks = report.ended_at_tick.saturating_sub(report.started_at_tick);
    let completed = report.proposals_accepted + report.proposals_rejected > 0
        || duration_ticks > 10;
    let icon = if completed { "●" } else { "⊘" };
    let icon_color = if completed {
        Color::from_rgb(0.45, 0.25, 0.7)
    } else {
        Color::from_rgb(0.85, 0.3, 0.3)
    };
    let fatigue_delta = report.fatigue_before - report.fatigue_after;

    column![
        row![
            text(icon).size(13).color(icon_color),
            text(format!(
                " Dream #{}  {:.0}% → {:.0}%  (Δ{:.0}%)",
                report.cycle_id,
                report.fatigue_before * 100.0,
                report.fatigue_after * 100.0,
                fatigue_delta * 100.0,
            )).size(13),
        ],
        text(format!(
            "  ticks {}-{}  duration: {}",
            report.started_at_tick, report.ended_at_tick, duration_ticks
        ))
        .size(11).color(Color::from_rgb(0.55, 0.55, 0.55)),
        text(format!(
            "  proposals: {} accepted, {} rejected  promotions: {}",
            report.proposals_accepted, report.proposals_rejected, report.sutra_written
        ))
        .size(11).color(Color::from_rgb(0.55, 0.55, 0.55)),
    ]
    .spacing(1)
    .padding([4u16, 8u16])
    .into()
}
