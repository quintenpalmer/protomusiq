use std::sync::mpsc;
use std::thread;

use crate::shared;

pub fn create_backend_with_client_and_callback() -> (
    shared::Client<shared::GUIToBackendMessage>,
    shared::Callback<shared::BackendToGUIMessage>,
) {
    let (sender_for_client, recv_for_backend) = mpsc::channel();

    let (callback_from_backend, callback_to_client) = mpsc::channel();

    thread::spawn(move || run_forever(recv_for_backend, callback_from_backend));

    (
        shared::Client::new(sender_for_client),
        shared::Callback::new(callback_to_client),
    )
}

pub fn run_forever(
    rx: mpsc::Receiver<shared::GUIToBackendMessage>,
    callback: mpsc::Sender<shared::BackendToGUIMessage>,
) {
    println!("MULTI-BACKEND:\tstarting to listen...");

    // TODO actually be the sink (and take on reporting and mpris functionality)

    println!("MULTI-BACKEND:\tdone listening");
}
