use iced::widget::{column, container, row, scrollable, text, Id};
use iced::{Element, Length};

use crate::app::App;

mod dumping;
use dumping:: error_box;

mod results;
use results::view_results;

mod keypad;
use keypad::view_keypad;

mod profile;
use profile::view_profile;

use crate::message::Message;

pub(super) const SINGLE_EL_WIDTH: u16 = 12;
pub(crate) const RESULTS_SCROLL: Id = Id::new("results");

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
            container(view_keypad()).height(350)
        );

    row![main_col, view_profile(app.get_current_profile().expect("msg"))].into()
}
