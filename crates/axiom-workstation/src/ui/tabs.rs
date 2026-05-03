use iced::widget::{button, row, text};
use iced::Element;

use crate::app::{Message, TabKind};

pub fn tabs_bar(active: TabKind, detached: &[TabKind]) -> Element<'static, Message> {
    let buttons: Vec<Element<Message>> = TabKind::all()
        .into_iter()
        .filter(|t| !detached.contains(t))
        .map(|tab| {
            button(text(tab.label()).size(13))
                .on_press(Message::TabSelected(tab))
                .style(if tab == active {
                    button::primary
                } else {
                    button::text
                })
                .into()
        })
        .collect();

    row(buttons).spacing(4).padding([0u16, 8u16]).into()
}
