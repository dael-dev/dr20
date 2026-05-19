use iced::{Event};

#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(Event),
    RollPress,
    StrPadPressed(String),
    InputChanged(String),
}