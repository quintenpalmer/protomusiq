use super::message;
use super::state;

pub fn backend_callback(app: &state::App) -> iced::Subscription<message::Message> {
    match app {
        state::App::Loaded(loaded) => iced::subscription::unfold(
            "backend message callback",
            loaded.player_info.backend_callback_recv.take(),
            move |mut callback| async move {
                let msg = callback.as_mut().unwrap().recv().unwrap();
                (message::Message::BackendCallback(msg), callback)
            },
        ),
        _ => {
            println!("CALLBACK:\tbackend subscription not started yet, app is not loaded");
            iced::Subscription::none()
        }
    }
}
