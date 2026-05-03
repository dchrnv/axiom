use iced::widget::{center, column, text};
use iced::{Element, Length};

use crate::app::Message;

pub fn placeholder_view(tab_name: &'static str) -> Element<'static, Message> {
    center(
        column![
            text(tab_name).size(24),
            text("Coming soon").size(14),
        ]
        .spacing(8)
        .align_x(iced::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
