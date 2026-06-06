use iced::widget::{container, text};
use iced::{Background, Color, Element, Theme};

use crate::message::Message;

//pub(super) const TOTAL_WIDTH: u16 = 240;

pub(super) fn error_box<'a>(message: &'a str) -> Element<'a, Message> {
    container(
        text(message)
            .size(16)
    )
    .padding(10)
    .style(|_theme: &Theme| container::Style {
        background: Some(Background::Color(Color::from_rgb8(80, 20, 20))),
        text_color: Some(Color::WHITE),
        border: iced::Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}
