use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, text, text_input,
};
use iced::{Color, Element, Length, Padding};

use crate::app::{ConversationMessage, ConversationState, Message, SystemMessageKind};

const DOMAINS: &[(u16, &str)] = &[
    (101, "EXEC"),
    (102, "SHDW"),
    (103, "CODX"),
    (104, "MAP"),
    (105, "PROB"),
    (106, "LOGI"),
    (107, "DREM"),
    (108, "ETHI"),
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

    scrollable(column(cards).spacing(4).padding([8u16, 12u16]))
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

            column![
                row![
                    text("You").size(12).color(Color::from_rgb(0.4, 0.65, 0.9)),
                    horizontal_space(),
                    text(domain_label)
                        .size(11)
                        .color(Color::from_rgb(0.5, 0.5, 0.5)),
                ],
                text(txt.as_str()).size(13),
                text(format_time(*timestamp_secs))
                    .size(10)
                    .color(Color::from_rgb(0.5, 0.5, 0.5)),
            ]
            .spacing(2)
            .padding(Padding {
                top: 4.0,
                right: 8.0,
                bottom: 4.0,
                left: 8.0,
            })
            .into()
        }
        ConversationMessage::System {
            text: txt,
            timestamp_secs,
            kind,
        } => {
            let (label, label_color) = match kind {
                SystemMessageKind::Acknowledgment => ("System", Color::from_rgb(0.5, 0.5, 0.5)),
                SystemMessageKind::FrameCreated => ("System", Color::from_rgb(0.3, 0.75, 0.4)),
                SystemMessageKind::FrameReactivated => ("System", Color::from_rgb(0.6, 0.5, 0.85)),
                SystemMessageKind::Error => ("System", Color::from_rgb(0.85, 0.3, 0.3)),
            };

            column![
                text(label).size(12).color(label_color),
                text(txt.as_str()).size(13).color(match kind {
                    SystemMessageKind::Error => Color::from_rgb(0.85, 0.3, 0.3),
                    _ => Color::from_rgb(0.85, 0.85, 0.85),
                }),
                text(format_time(*timestamp_secs))
                    .size(10)
                    .color(Color::from_rgb(0.5, 0.5, 0.5)),
            ]
            .spacing(2)
            .padding(Padding {
                top: 4.0,
                right: 8.0,
                bottom: 4.0,
                left: 8.0,
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
            button(text(*name).size(11))
                .on_press(Message::ConversationDomainSelected(*id))
                .style(if selected {
                    button::primary
                } else {
                    button::secondary
                })
                .into()
        })
        .collect();

    let can_submit = !conv.sending && !conv.input_buffer.trim().is_empty();

    let submit_label = if conv.sending { "Sending..." } else { "Submit" };
    let submit_btn = button(text(submit_label).size(13))
        .on_press_maybe(if can_submit {
            Some(Message::ConversationSubmit)
        } else {
            None
        })
        .style(button::primary);

    column![
        text("Target domain:")
            .size(11)
            .color(Color::from_rgb(0.5, 0.5, 0.5)),
        row(domain_btns).spacing(4),
        row![
            text_input(
                "Enter text and press Enter or Submit...",
                &conv.input_buffer
            )
            .on_input(Message::ConversationInputChanged)
            .on_submit(Message::ConversationSubmit)
            .size(13)
            .width(Length::Fill),
            submit_btn,
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center),
    ]
    .spacing(6)
    .padding(12)
    .into()
}

// ── Helpers ────────────────────────────────────────────────────────────────

fn format_time(secs: u64) -> String {
    let s = secs % 60;
    let m = (secs / 60) % 60;
    let h = (secs / 3600) % 24;
    format!("{:02}:{:02}:{:02}", h, m, s)
}
