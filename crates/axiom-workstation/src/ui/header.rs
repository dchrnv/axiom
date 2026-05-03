use iced::widget::{horizontal_space, row, text};
use iced::Element;

use crate::app::{ConnectionState, Message};

pub fn header_view(connection: &ConnectionState) -> Element<'_, Message> {
    let conn_text = match connection {
        ConnectionState::Disconnected => "● Disconnected".to_string(),
        ConnectionState::Connecting => "● Connecting...".to_string(),
        ConnectionState::Reconnecting { attempt, next_retry_secs } => {
            format!("● Reconnecting (attempt {}, {}s)", attempt, next_retry_secs)
        }
        ConnectionState::Connected { engine_version, connected_at } => {
            format!(
                "● Connected  v{:#010x}  ({}s)",
                engine_version,
                connected_at.elapsed().as_secs()
            )
        }
    };

    row![
        text("AXIOM Workstation").size(16),
        horizontal_space(),
        text(conn_text).size(14),
    ]
    .padding(8)
    .into()
}
