use iced::widget::{column, container, row, scrollable, text};
use iced::{Color, Element, Length};

use axiom_protocol::bench::BenchResults;

use crate::app::{BenchmarksState, Message};

pub fn benchmarks_view<'a>(state: &'a BenchmarksState) -> Element<'a, Message> {
    column![running_panel(state), history_panel(state),]
        .spacing(0)
        .into()
}

// ── Running bench ──────────────────────────────────────────────────────────

fn running_panel<'a>(state: &'a BenchmarksState) -> Element<'a, Message> {
    let Some(ref rb) = state.running else {
        return column![].into();
    };

    let progress_label = if rb.total > 0 {
        format!(
            "{} / {}  ({:.0}%)",
            rb.completed,
            rb.total,
            rb.completed as f64 / rb.total as f64 * 100.0
        )
    } else {
        format!("{} iterations done", rb.completed)
    };

    container(
        column![
            row![
                text("◌").size(13).color(Color::from_rgb(0.4, 0.65, 0.9)),
                text(format!(" Running: {}  run #{}", rb.bench_id, rb.run_id)).size(13),
            ],
            text(progress_label)
                .size(11)
                .color(Color::from_rgb(0.55, 0.55, 0.55)),
        ]
        .spacing(6),
    )
    .padding([8, 16])
    .into()
}

// ── History ────────────────────────────────────────────────────────────────

fn history_panel<'a>(state: &'a BenchmarksState) -> Element<'a, Message> {
    if state.history.is_empty() {
        return container(
            column![
                text("Benchmarks")
                    .size(13)
                    .color(Color::from_rgb(0.6, 0.6, 0.6)),
                text("No benchmark results yet.")
                    .size(13)
                    .color(Color::from_rgb(0.45, 0.45, 0.45)),
            ]
            .spacing(8),
        )
        .padding(16)
        .height(Length::Fill)
        .into();
    }

    let cards: Vec<Element<Message>> = state.history.iter().map(bench_card).collect();

    container(
        column![
            text("Benchmark results")
                .size(13)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
            scrollable(column(cards).spacing(4)).height(Length::Fill),
        ]
        .spacing(8),
    )
    .padding(16)
    .height(Length::Fill)
    .into()
}

fn bench_card<'a>(r: &'a BenchResults) -> Element<'a, Message> {
    fn fmt_ns(ns: f64) -> String {
        if ns >= 1_000_000.0 {
            format!("{:.2} ms", ns / 1_000_000.0)
        } else if ns >= 1_000.0 {
            format!("{:.2} µs", ns / 1_000.0)
        } else {
            format!("{:.0} ns", ns)
        }
    }

    column![
        row![
            text("●").size(13).color(Color::from_rgb(0.4, 0.65, 0.9)),
            text(format!(" {}  ×{}", r.bench_id, r.iterations)).size(13),
        ],
        text(format!(
            "  median: {}  p99: {}  σ: {}",
            fmt_ns(r.median_ns),
            fmt_ns(r.p99_ns),
            fmt_ns(r.std_dev_ns),
        ))
        .size(11)
        .color(Color::from_rgb(0.55, 0.55, 0.55)),
        text(format!(
            "  {}  {}  v{:#010x}",
            r.environment.os, r.environment.arch, r.environment.engine_version
        ))
        .size(10)
        .color(Color::from_rgb(0.4, 0.4, 0.4)),
    ]
    .spacing(1)
    .padding([4u16, 8u16])
    .into()
}
