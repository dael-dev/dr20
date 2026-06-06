use iced::widget::{column, row, text};
use iced::{Element};

use crate::app::App;
use crate::app::RollResult;

use crate::message::Message;

pub(super) fn view_results(app: &App) -> Element<'_, Message>
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