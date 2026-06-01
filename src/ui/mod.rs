
use iced::widget::{Space, button, column, container, row, scrollable, text, Grid, Id};
use iced::{Background, Color, Element, Length, Theme};

use crate::app::App;

mod dumping;
use dumping::{view_results, view_profile, view_key_pad, error_box};

use crate::message::Message;

pub const RESULTS_SCROLL: Id = Id::new("results");

pub fn view(app: &App) -> Element<'_, Message> {
    let (left, right) = app.prompt_parts();
    
    let mut main_col = column![
            scrollable(view_results(app))
                .height(Length::Fill)
                .width(Length::Fill)
                .id(RESULTS_SCROLL.clone())
        ]
        .padding(20)
        .spacing(10);
 
    if let Some(error_string) = app.get_error() {
        main_col = main_col
            .push(error_box(error_string));
    }

    main_col = main_col
        .push(
            text(format!("{}|{}", left, right))
        )
        .push(
            container(view_key_pad()).height(350)
        );

    row![main_col, view_profile(app.get_current_profile().expect("msg"))].into()
}
