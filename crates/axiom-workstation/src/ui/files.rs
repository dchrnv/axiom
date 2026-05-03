use iced::widget::{button, column, container, horizontal_space, row, scrollable, text, text_input};
use iced::{Color, Element, Length};

use crate::app::{CompletedImport, FilesState, Message};

pub fn files_view<'a>(state: &'a FilesState) -> Element<'a, Message> {
    column![
        import_controls(state),
        progress_section(state),
        history_section(state),
    ]
    .spacing(0)
    .into()
}

// ── Import controls ────────────────────────────────────────────────────────

fn import_controls<'a>(state: &'a FilesState) -> Element<'a, Message> {
    let adapter_row: Element<'a, Message> = if state.adapters_fetched {
        if state.available_adapters.is_empty() {
            row![text("No adapters available.")
                .size(13)
                .color(Color::from_rgb(0.5, 0.5, 0.5))]
            .into()
        } else {
            let buttons: Vec<Element<Message>> = state
                .available_adapters
                .iter()
                .map(|a| {
                    let selected = state
                        .selected_adapter_id
                        .as_deref()
                        .map(|id| id == a.id)
                        .unwrap_or(false);
                    let btn = button(text(&a.name).size(12));
                    let btn = if selected {
                        btn.style(button::primary)
                    } else {
                        btn.style(button::secondary)
                    };
                    btn.on_press(Message::FilesAdapterSelected(a.id.clone())).into()
                })
                .collect();
            row(buttons).spacing(6).into()
        }
    } else {
        row![text("Loading adapters…")
            .size(13)
            .color(Color::from_rgb(0.5, 0.5, 0.5))]
        .into()
    };

    let path_input = text_input("Path to source file or directory…", &state.source_path)
        .on_input(Message::FilesPathChanged)
        .size(13)
        .padding(8);

    let can_start = state.selected_adapter_id.is_some()
        && !state.source_path.is_empty()
        && state.running_import.is_none();

    let start_btn = if can_start {
        button(text("Start import").size(12))
            .on_press(Message::FilesStartImport)
            .style(button::primary)
    } else {
        button(text("Start import").size(12)).style(button::primary)
    };

    container(
        column![
            text("Knowledge import").size(13).color(Color::from_rgb(0.6, 0.6, 0.6)),
            text("Adapter").size(11).color(Color::from_rgb(0.5, 0.5, 0.5)),
            adapter_row,
            text("Source path").size(11).color(Color::from_rgb(0.5, 0.5, 0.5)),
            path_input,
            start_btn,
        ]
        .spacing(8),
    )
    .padding(16)
    .into()
}

// ── Progress ───────────────────────────────────────────────────────────────

fn progress_section<'a>(state: &'a FilesState) -> Element<'a, Message> {
    let Some(ref ri) = state.running_import else {
        return column![].into();
    };

    let progress_label = if ri.total > 0 {
        format!(
            "{} / {} ({:.0}%)",
            ri.processed,
            ri.total,
            ri.processed as f64 / ri.total as f64 * 100.0
        )
    } else {
        format!("{} processed", ri.processed)
    };

    let action: Element<Message> = if state.cancel_confirm {
        row![
            text("Cancel import?").size(12),
            horizontal_space(),
            button(text("No").size(12))
                .on_press(Message::FilesCancelDismiss)
                .style(button::secondary),
            button(text("Cancel import").size(12))
                .on_press(Message::FilesConfirmCancel)
                .style(button::danger),
        ]
        .spacing(6)
        .into()
    } else {
        button(text("Cancel").size(12))
            .on_press(Message::FilesCancelRequest)
            .style(button::secondary)
            .into()
    };

    container(
        column![
            row![
                text("●").size(13).color(Color::from_rgb(0.4, 0.65, 0.9)),
                text(format!(" Importing via {}  —  {}", ri.adapter_id, ri.source)).size(13),
            ],
            row![
                text(progress_label)
                    .size(11)
                    .color(Color::from_rgb(0.55, 0.55, 0.55)),
                horizontal_space(),
                action,
            ]
            .spacing(8),
        ]
        .spacing(6),
    )
    .padding([8, 16])
    .into()
}

// ── History ────────────────────────────────────────────────────────────────

fn history_section<'a>(state: &'a FilesState) -> Element<'a, Message> {
    if state.completed_imports.is_empty() {
        return container(
            text("No completed imports.")
                .size(13)
                .color(Color::from_rgb(0.45, 0.45, 0.45)),
        )
        .padding([0, 16])
        .into();
    }

    let cards: Vec<Element<Message>> = state
        .completed_imports
        .iter()
        .map(import_card)
        .collect();

    container(
        column![
            text("Import history")
                .size(13)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
            scrollable(column(cards).spacing(4)).height(Length::Fill),
        ]
        .spacing(8),
    )
    .padding([0, 16])
    .height(Length::Fill)
    .into()
}

fn import_card<'a>(imp: &'a CompletedImport) -> Element<'a, Message> {
    let (icon, icon_color) = if imp.cancelled {
        ("⊗", Color::from_rgb(0.85, 0.3, 0.3))
    } else if imp.errors > 0 {
        ("▲", Color::from_rgb(0.9, 0.65, 0.1))
    } else {
        ("●", Color::from_rgb(0.3, 0.75, 0.4))
    };

    column![
        row![
            text(icon).size(13).color(icon_color),
            text(format!(" {}  →  {}", imp.adapter_id, imp.source)).size(13),
        ],
        text(format!(
            "  tokens: {}  errors: {}",
            imp.tokens_added, imp.errors
        ))
        .size(11)
        .color(Color::from_rgb(0.55, 0.55, 0.55)),
    ]
    .spacing(1)
    .padding([4u16, 8u16])
    .into()
}
