use iced::{event, executor, keyboard, Application, Command, Element};

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
    type Theme = iced::Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        init::init_app()
    }

    fn title(&self) -> String {
        String::from("Musiq Library")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        update::update(self, message)
    }

    fn view(&self) -> Element<Self::Message> {
        match self {
            state::App::Loading => view::view_loading(),
            state::App::Loaded(ref loaded) => view::view_app(loaded),
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::batch(vec![
            subscription::backend_callback(self),
            keyboard::on_key_press(subscription::keybinding_subscription_fn),
            event::listen_with(|event, _| match event {
                event::Event::Mouse(mouse_event) => subscription::on_mouse_event(mouse_event),
                _ => None,
            }),
        ])
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn style(&self) -> iced::theme::Application {
        fn dark_background(_theme: &iced::theme::Theme) -> iced::application::Appearance {
            iced::application::Appearance {
                background_color: iced::Color::from_rgb8(0x15, 0x15, 0x15),
                text_color: iced::Color::WHITE,
            }
        }

        iced::theme::Application::custom(dark_background)
    }

    fn scale_factor(&self) -> f64 {
        match self {
            state::App::Loaded(loaded) => loaded.config.rest.scale_factor,
            state::App::Loading => 1.0,
        }
    }

    // TODO replace with `iced_native::window::Action`
    //fn should_exit(&self) -> bool {
    //    match self {
    //        state::App::Loaded(loaded) => loaded.rest.should_close,
    //        _ => false,
    //    }
    //}
}
