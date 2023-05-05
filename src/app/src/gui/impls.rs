use iced::{self, executor, Application, Command, Element};

use super::init;
use super::message;
use super::state;
use super::subscription;
use super::update;
use super::view;

impl Application for state::App {
    type Executor = executor::Default;
    type Message = message::Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        init::init_app()
    }

    fn title(&self) -> String {
        String::from("Musiq Library")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> Command<Self::Message> {
        match self {
            state::App::Loading => update::update_from_loading_state(self, message),
            state::App::Loaded(ref mut loaded) => update::update_state(loaded, message),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        match self {
            state::App::Loading => view::view_loading(),
            state::App::Loaded(ref mut loaded) => view::view_app(loaded),
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::batch(vec![
            subscription::sink_callback(&self),
            subscription::mpris_callback(&self),
        ])
    }

    fn background_color(&self) -> iced::Color {
        iced::Color::from_rgb8(0x15, 0x15, 0x15)
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn should_exit(&self) -> bool {
        match self {
            state::App::Loaded(loaded) => loaded.rest.should_close,
            _ => false,
        }
    }
}
