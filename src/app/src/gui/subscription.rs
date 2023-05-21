use super::message;
use super::state;

pub fn mpris_callback(app: &state::App) -> iced::Subscription<message::Message> {
    match app {
        state::App::Loaded(loaded) => iced::subscription::unfold(
            "mpris message callback",
            loaded.player_info.mpris_callback_recv.take(),
            move |mut callback| async move {
                let msg = callback.as_mut().unwrap().recv().unwrap();
                (message::Message::MprisCallback(msg), callback)
            },
        ),
        _ => {
            println!("CALLBACK:\tmpris subscription not started yet, app is not loaded");
            iced::Subscription::none()
        }
    }
}

pub fn sink_callback(app: &state::App) -> iced::Subscription<message::Message> {
    match app {
        state::App::Loaded(loaded) => iced::subscription::unfold(
            "sink message callback",
            loaded.player_info.sink_callback_recv.take(),
            move |mut callback| async move {
                let msg = callback.as_mut().unwrap().recv().unwrap();
                (message::Message::SinkCallback(msg), callback)
            },
        ),
        _ => {
            println!("CALLBACK:\tmpris subscription not started yet, app is not loaded");
            iced::Subscription::none()
        }
    }
}
