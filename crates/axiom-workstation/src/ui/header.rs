use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Color, Element, Length};

use crate::app::{ConnectionState, Message};

pub fn header_view<'a>(
    connection: &'a ConnectionState,
    show_details: bool,
    engine_address: &'a str,
) -> Element<'a, Message> {
    let (indicator, conn_text) = match connection {
        ConnectionState::Disconnected => ("●", "Disconnected".to_string()),
        ConnectionState::Connecting => ("●", "Connecting…".to_string()),
        ConnectionState::Reconnecting {
            attempt,
            next_retry_secs,
        } => (
            "●",
            format!("Reconnecting (attempt {}, {}s)", attempt, next_retry_secs),
        ),
        ConnectionState::Connected {
            engine_version,
            connected_at,
        } => (
            "●",
            format!(
                "Connected  v{:#010x}  ({}s)",
                engine_version,
                connected_at.elapsed().as_secs()
            ),
        ),
    };

    let indicator_color = match connection {
        ConnectionState::Connected { .. } => Color::from_rgb(0.35, 0.7, 0.45),
        ConnectionState::Reconnecting { .. } => Color::from_rgb(0.85, 0.65, 0.2),
        _ => Color::from_rgb(0.8, 0.35, 0.3),
    };

    let conn_btn = button(
        row![
            text(indicator).size(14).color(indicator_color),
            text(format!("  {}", conn_text)).size(13),
        ]
        .align_y(iced::Alignment::Center),
    )
    .on_press(Message::ToggleConnectionDetails)
    .style(button::text);

    let top_bar = row![
        text("AXIOM Workstation").size(15),
        horizontal_space(),
        conn_btn,
    ]
    .padding(8)
    .align_y(iced::Alignment::Center);

    if show_details {
        column![top_bar, connection_details(connection, engine_address)].into()
    } else {
        top_bar.into()
    }
}

fn connection_details<'a>(
    connection: &'a ConnectionState,
    engine_address: &'a str,
) -> Element<'a, Message> {
    let details: Element<Message> = match connection {
        ConnectionState::Connected {
            engine_version,
            connected_at,
        } => column![
            text(format!("Engine address:  {}", engine_address))
                .size(12)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
            text(format!("Engine version:  v{:#010x}", engine_version))
                .size(12)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
            text(format!(
                "Connected for:   {}s",
                connected_at.elapsed().as_secs()
            ))
            .size(12)
            .color(Color::from_rgb(0.6, 0.6, 0.6)),
        ]
        .spacing(3)
        .into(),
        _ => column![
            text(format!("Engine address:  {}", engine_address))
                .size(12)
                .color(Color::from_rgb(0.6, 0.6, 0.6)),
            text("Not connected")
                .size(12)
                .color(Color::from_rgb(0.7, 0.4, 0.35)),
        ]
        .spacing(3)
        .into(),
    };

    container(
        row![
            details,
            horizontal_space(),
            button(text("Disconnect").size(12))
                .on_press(Message::WsDisconnected)
                .style(button::secondary),
        ]
        .spacing(16)
        .align_y(iced::Alignment::Center),
    )
    .padding([4, 16])
    .width(Length::Fill)
    .style(|theme| {
        let base = container::rounded_box(theme);
        container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.09, 0.09, 0.09))),
            ..base
        }
    })
    .into()
}
