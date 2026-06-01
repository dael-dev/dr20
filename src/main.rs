use iced::{Size, Subscription, Theme, event, window};

mod app;
use app::App;

mod message;
use message::Message;

mod ui;
mod profile;

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, ui::view)
        .window(window::Settings {
            size: Size::new(512.0, 700.0),
            min_size: Some(Size::new(245.0, 400.0)),
            ..Default::default()
        })
        .theme(Theme::Moonfly)
        .centered()
        .subscription(subscription)
        .run()
}

fn subscription(_app: &App) -> Subscription<Message> {
    event::listen().map(Message::EventOccurred)
}
