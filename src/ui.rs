use iced::widget::{Space, button, column, container, row, scrollable, text};
use iced::{Element, Length};


use crate::app::App;
use crate::app::RollResult;

use crate::message::Message;

pub const TOTAL_WIDTH: u16 = 240;
pub const SINGLE_EL_WIDTH: u16 = 12;

pub fn view(app: &App) -> Element<'_, Message> {
    column![
        scrollable(view_results(app))
            .height(Length::Fill)
            .width(Length::Fill),
        text(app.render_output_str()),
        container(view_key_pad())
            .height(350)
    ]
    .padding(20)
    .spacing(10)
    .into()
}

fn view_results(app: &App) -> Element<Message>
{
    column![
        app.results
        .iter()
        .map(result_roll)
        .fold(
            column!().spacing(4),
            |col, el| col.push(el),
        )
    ]
    .spacing(4)
    .into()
}

fn result_roll(result: &RollResult)-> Element<'_, Message> {

    row![
        text(result.value.to_string())
            .width(80)
            .size(24),

        text(
            result.text.clone()
        )
    ]
    .spacing(12)
    .padding(8)
    .into()
}

fn view_key_pad() -> Element<'static, Message> {
    column![
        row![
            keypad_button("Max", ButtonWidth::OneAndHalf, ButtonHeight::TwoThirds),
            keypad_button("Min", ButtonWidth::OneAndHalf, ButtonHeight::TwoThirds),
            Space::new().width(Length::Fixed(2.0)),
            Space::new().width(Length::FillPortion(ButtonWidth::Double.value())),
  
        ].spacing(4),

        row![
            keypad_button("kh", ButtonWidth::Single,ButtonHeight::TwoThirds),
            keypad_button("r", ButtonWidth::Single, ButtonHeight::TwoThirds),
            keypad_button("x", ButtonWidth::Single, ButtonHeight::TwoThirds),
            Space::new().width(Length::Fixed(2.0)),
            keypad_button("⌫", ButtonWidth::Single, ButtonHeight::TwoThirds),
            keypad_button("CLR", ButtonWidth::Single, ButtonHeight::TwoThirds),  
        ].spacing(4),

        row![
            keypad_button("kl", ButtonWidth::Single, ButtonHeight::TwoThirds),
            keypad_button("rr", ButtonWidth::Single, ButtonHeight::TwoThirds),
            keypad_button("x0", ButtonWidth::Single, ButtonHeight::TwoThirds),
            Space::new().width(Length::Fixed(2.0)),
            keypad_button("←", ButtonWidth::Single, ButtonHeight::TwoThirds),
            keypad_button("→", ButtonWidth::Single, ButtonHeight::TwoThirds),  
        ].spacing(4),

        row![
            keypad_button(">", ButtonWidth::Single, ButtonHeight::TwoThirds),
            keypad_button("<", ButtonWidth::Single, ButtonHeight::TwoThirds),
            keypad_button("=", ButtonWidth::Single, ButtonHeight::TwoThirds),
            Space::new().width(Length::Fixed(2.0)),
            keypad_button("(", ButtonWidth::Single, ButtonHeight::TwoThirds),
            keypad_button(")", ButtonWidth::Single, ButtonHeight::TwoThirds),  
        ].spacing(4),

        Space::new().height(4),

        row![
            keypad_button("7", ButtonWidth::Single, ButtonHeight::Single),
            keypad_button("8", ButtonWidth::Single, ButtonHeight::Single),
            keypad_button("9", ButtonWidth::Single, ButtonHeight::Single),
            Space::new().width(Length::Fixed(2.0)),
            keypad_button("/", ButtonWidth::Single, ButtonHeight::Single),
            keypad_button("\\", ButtonWidth::Single, ButtonHeight::Single),  
        ].spacing(4),

        row![
            keypad_button("4", ButtonWidth::Single, ButtonHeight::Single),
            keypad_button("5", ButtonWidth::Single, ButtonHeight::Single),
            keypad_button("6", ButtonWidth::Single, ButtonHeight::Single),
            Space::new().width(Length::Fixed(2.0)),
            keypad_button("*", ButtonWidth::Single, ButtonHeight::Single),
            keypad_button("", ButtonWidth::Single, ButtonHeight::Single),
        ].spacing(4),

        row![
            keypad_button("1", ButtonWidth::Single, ButtonHeight::Single),
            keypad_button("2", ButtonWidth::Single, ButtonHeight::Single),
            keypad_button("3", ButtonWidth::Single, ButtonHeight::Single),
            Space::new().width(Length::Fixed(2.0)),
            keypad_button("+", ButtonWidth::Single, ButtonHeight::Single),
            keypad_button("-", ButtonWidth::Single, ButtonHeight::Single),
        ].spacing(4),

        row![
            keypad_button("0", ButtonWidth::Double,ButtonHeight::Single),
            keypad_button("d", ButtonWidth::Single, ButtonHeight::Single),
            Space::new().width(Length::Fixed(2.0)),
            keypad_button("ROLL", ButtonWidth::Double, ButtonHeight::Single),
        ].spacing(4),
    ]
    .spacing(4)
    .width(Length::Fixed(480.0))
    .height(Length::Fixed(600.0))
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonWidth {
    Single,
    OneAndHalf,
    Double,
}

impl ButtonWidth {
    pub fn value(self) -> u16 {
        match self {
            Self::Single => 2 * SINGLE_EL_WIDTH,
            Self::OneAndHalf => 3 * SINGLE_EL_WIDTH,
            Self::Double => 4 * SINGLE_EL_WIDTH,
        }
    }
}

pub const BUTTON_HEIGHT_BASE: f32 = 48.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonHeight{
    TwoThirds,
    Single,
    OneAndHalf,
    Double,
}

impl ButtonHeight {
    pub fn value(self) -> Length {
        match self {
            Self::TwoThirds => Length::Fixed(2.0 * BUTTON_HEIGHT_BASE / 3.0),
            Self::Single => Length::Fixed(BUTTON_HEIGHT_BASE),
            Self::OneAndHalf => Length::Fixed(1.5 * BUTTON_HEIGHT_BASE),
            Self::Double => Length::Fixed(2.0 * BUTTON_HEIGHT_BASE),
        }
    }
}

fn keypad_button(label: &'static str, width: ButtonWidth, height: ButtonHeight) -> Element<'static, Message> {
    button(text(label).center())
        .width(Length::FillPortion(width.value()))
        .height(height.value())
        .on_press(Message::StrPadPressed(label.to_string()))
        .into()
}