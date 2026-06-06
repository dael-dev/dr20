use iced::{Event};
use crate::profile::ExprId;

#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(Event),
    StrPadPressed(String),
    RollExpression(ExprId),
}