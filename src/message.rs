use iced::{Event, widget::Id};

#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(Event),
    RollPress,
    StrPadPressed(String),
}