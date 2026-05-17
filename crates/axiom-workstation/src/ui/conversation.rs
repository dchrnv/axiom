use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, text, text_editor,
};
use iced::{Border, Color, Element, Length, Padding};

use crate::app::{ConversationMessage, ConversationState, Message, SystemMessageKind};

const DOMAINS: &[(u16, &str)] = &[
    (101, "EXEC"),
    (102, "SHDW"),
    (103, "CODX"),
    (104, "MAP"),
    (105, "PROB"),
    (106, "LOGI"),
    (107, "DREM"),
    (108, "VOID"),
];

pub fn conversation_view<'a>(conv: &'a ConversationState) -> Element<'a, Message> {
    column![message_feed(&conv.messages), input_panel(conv),].into()
}

// ── Message feed ───────────────────────────────────────────────────────────

fn message_feed<'a>(messages: &'a [ConversationMessage]) -> Element<'a, Message> {
    if messages.is_empty() {
        return container(
            text("No messages yet. Submit text to start.")
                .size(13)
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
        )
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into();
    }

    let cards: Vec<Element<Message>> = messages.iter().map(message_card).collect();

    scrollable(column(cards).spacing(6).padding([8u16, 12u16]))
        .id(iced::widget::scrollable::Id::new("chat_feed"))
        .height(Length::Fill)
        .into()
}

fn message_card<'a>(msg: &'a ConversationMessage) -> Element<'a, Message> {
    match msg {
        ConversationMessage::User {
            text: txt,
            target_domain,
            timestamp_secs,
        } => {
            let domain_label = DOMAINS
                .iter()
                .find(|(id, _)| *id == *target_domain)
                .map(|(_, name)| *name)
                .unwrap_or("?");

            let inner = column![
                row![
                    text("You").size(12).color(Color::from_rgb(0.45, 0.68, 0.92)),
                    horizontal_space(),
                    text(domain_label)
                        .size(11)
                        .color(Color::from_rgb(0.45, 0.45, 0.55)),
                ],
                text(txt.as_str()).size(13),
                text(format_time(*timestamp_secs))
                    .size(10)
                    .color(Color::from_rgb(0.45, 0.45, 0.55)),
            ]
            .spacing(3);

            container(inner)
                .padding(Padding { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 })
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgba(
                        0.15, 0.22, 0.35, 0.5,
                    ))),
                    border: Border {
                        color: Color::from_rgba(0.35, 0.5, 0.8, 0.25),
                        width: 1.0,
                        radius: 6.0.into(),
                    },
                    ..container::Style::default()
                })
                .into()
        }
        ConversationMessage::System {
            text: txt,
            timestamp_secs,
            kind,
        } => {
            let (label, label_color, bg, border_color) = match kind {
                SystemMessageKind::Acknowledgment => (
                    "Engine",
                    Color::from_rgb(0.5, 0.5, 0.55),
                    Color::from_rgba(0.12, 0.12, 0.14, 0.4),
                    Color::from_rgba(0.3, 0.3, 0.35, 0.2),
                ),
                SystemMessageKind::FrameCreated => (
                    "Frame created",
                    Color::from_rgb(0.3, 0.78, 0.45),
                    Color::from_rgba(0.05, 0.2, 0.1, 0.4),
                    Color::from_rgba(0.2, 0.6, 0.3, 0.25),
                ),
                SystemMessageKind::FrameReactivated => (
                    "Frame reactivated",
                    Color::from_rgb(0.6, 0.5, 0.88),
                    Color::from_rgba(0.12, 0.08, 0.22, 0.4),
                    Color::from_rgba(0.45, 0.35, 0.75, 0.25),
                ),
                SystemMessageKind::Error => (
                    "Error",
                    Color::from_rgb(0.88, 0.35, 0.35),
                    Color::from_rgba(0.25, 0.06, 0.06, 0.4),
                    Color::from_rgba(0.7, 0.25, 0.25, 0.3),
                ),
            };

            let inner = column![
                text(label).size(11).color(label_color),
                text(txt.as_str()).size(13).color(match kind {
                    SystemMessageKind::Error => Color::from_rgb(0.9, 0.45, 0.45),
                    _ => Color::from_rgb(0.82, 0.82, 0.85),
                }),
                text(format_time(*timestamp_secs))
                    .size(10)
                    .color(Color::from_rgb(0.4, 0.4, 0.45)),
            ]
            .spacing(3);

            container(inner)
                .padding(Padding { top: 6.0, right: 12.0, bottom: 6.0, left: 12.0 })
                .style(move |_theme| container::Style {
                    background: Some(iced::Background::Color(bg)),
                    border: Border {
                        color: border_color,
                        width: 1.0,
                        radius: 6.0.into(),
                    },
                    ..container::Style::default()
                })
                .into()
        }
    }
}

// ── Input panel ────────────────────────────────────────────────────────────

fn input_panel<'a>(conv: &'a ConversationState) -> Element<'a, Message> {
    let domain_btns: Vec<Element<Message>> = DOMAINS
        .iter()
        .map(|(id, name)| {
            let selected = *id == conv.target_domain;
            button(text(*name).size(10))
                .on_press(Message::ConversationDomainSelected(*id))
                .padding([3u16, 7u16])
                .style(if selected {
                    button::primary
                } else {
                    button::secondary
                })
                .into()
        })
        .collect();

    let can_submit = !conv.sending && !conv.editor_content.text().trim().is_empty();

    let submit_label: &str = if conv.sending { "Sending…" } else { "Submit" };

    let submit_btn = button(text(submit_label).size(13))
        .on_press_maybe(if can_submit {
            Some(Message::ConversationSubmit)
        } else {
            None
        })
        .style(button::primary);

    let editor = text_editor(&conv.editor_content)
        .on_action(Message::ConversationEditorAction)
        .placeholder("Enter text… (Ctrl+Enter to submit)")
        .height(100)
        .key_binding(|kp| {
            use iced::keyboard::key::Named;
            use iced::keyboard::Key;
            if kp.modifiers.control() && kp.key == Key::Named(Named::Enter) {
                Some(text_editor::Binding::Custom(Message::ConversationSubmit))
            } else {
                text_editor::Binding::from_key_press(kp)
            }
        });

    let domain_row = row![
        text("→").size(11).color(Color::from_rgb(0.45, 0.45, 0.55)),
        row(domain_btns).spacing(3),
    ]
    .spacing(6)
    .align_y(iced::Alignment::Center);

    container(
        column![
            domain_row,
            editor,
            row![horizontal_space(), submit_btn],
        ]
        .spacing(6),
    )
    .padding(12)
    .style(|_theme| container::Style {
        background: Some(iced::Background::Color(Color::from_rgba(0.06, 0.06, 0.08, 1.0))),
        border: Border {
            color: Color::from_rgba(0.2, 0.2, 0.25, 0.4),
            width: 1.0,
            radius: 0.0.into(),
        },
        ..container::Style::default()
    })
    .into()
}

// ── Helpers ────────────────────────────────────────────────────────────────

fn format_time(secs: u64) -> String {
    let s = secs % 60;
    let m = (secs / 60) % 60;
    let h = (secs / 3600) % 24;
    format!("{:02}:{:02}:{:02}", h, m, s)
}
