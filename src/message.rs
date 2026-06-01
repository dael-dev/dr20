use iced::{Event, widget::Id};

#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(Event),
    RollPress,
    StrPadPressed(String),
}

pub const RESULTS_SCROLL: Id = Id::new("results");