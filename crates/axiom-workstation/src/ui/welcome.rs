use iced::widget::{button, column, container, text};
use iced::{Color, Element, Length};

use crate::app::{ConnectionState, Message, TabKind};

pub fn welcome_view<'a>(
    connection: &'a ConnectionState,
    opacity: f32,
    phase: f32,
) -> Element<'a, Message> {
    let a = opacity.clamp(0.0, 1.0);
    let status = connection_status(connection, a, phase);

    container(
        column![
            text("AXIOM")
                .size(52)
                .color(Color { a, ..Color::from_rgb(0.75, 0.75, 0.75) }),
            text("Workstation")
                .size(28)
                .color(Color { a, ..Color::from_rgb(0.55, 0.55, 0.55) }),
            status,
        ]
        .spacing(24)
        .align_x(iced::Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

fn spinner_frame(phase: f32) -> &'static str {
    const FRAMES: &[&str] = &["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];
    let idx = (phase * FRAMES.len() as f32) as usize % FRAMES.len();
    FRAMES[idx]
}

fn connection_status<'a>(
    connection: &'a ConnectionState,
    a: f32,
    phase: f32,
) -> Element<'a, Message> {
    match connection {
        ConnectionState::Connecting => column![
            text("Connecting to engine…")
                .size(14)
                .color(Color { a, ..Color::from_rgb(0.5, 0.5, 0.5) }),
            text(spinner_frame(phase))
                .size(18)
                .color(Color { a, ..Color::from_rgb(0.4, 0.5, 0.7) }),
        ]
        .spacing(8)
        .align_x(iced::Alignment::Center)
        .into(),

        ConnectionState::Reconnecting {
            attempt,
            next_retry_secs,
        } => column![
            text(format!(
                "Reconnecting… (attempt {}, retry in {}s)",
                attempt, next_retry_secs
            ))
            .size(13)
            .color(Color { a, ..Color::from_rgb(0.75, 0.6, 0.3) }),
            text(spinner_frame(phase))
                .size(18)
                .color(Color { a, ..Color::from_rgb(0.7, 0.55, 0.3) }),
        ]
        .spacing(6)
        .align_x(iced::Alignment::Center)
        .into(),

        ConnectionState::Disconnected => column![
            text("Engine is not running")
                .size(15)
                .color(Color { a, ..Color::from_rgb(0.8, 0.4, 0.3) }),
            text("To use Workstation, the engine must be started.")
                .size(13)
                .color(Color { a, ..Color::from_rgb(0.55, 0.55, 0.55) }),
            button(text("Configure connection").size(13))
                .on_press(Message::TabSelected(TabKind::Configuration))
                .style(button::secondary),
            button(text("Wait for engine").size(13))
                .on_press(Message::SkipToMain)
                .style(button::secondary),
        ]
        .spacing(10)
        .align_x(iced::Alignment::Center)
        .into(),

        ConnectionState::Connected { .. } => column![text("Connected — starting…")
            .size(14)
            .color(Color { a, ..Color::from_rgb(0.35, 0.7, 0.45) }),]
        .align_x(iced::Alignment::Center)
        .into(),
    }
}
