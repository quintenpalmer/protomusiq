mod backend;
mod remotebridge;
mod remotehandler;

use std::sync::mpsc;
use std::thread;

use crate::shared;
use crate::shared::{Callback, Client};

pub fn create_backend_with_client_and_callback() -> (
    Client<shared::SinkMessage>,
    Callback<shared::SinkCallbackMessage>,
) {
    let (sender_for_client, recv_for_backend) = mpsc::channel();

    let (callback_from_backend, callback_to_client) = mpsc::channel();

    thread::spawn(move || backend::run_forever(recv_for_backend, callback_from_backend));

    (
        Client::new(sender_for_client),
        Callback::new(callback_to_client),
    )
}

pub fn create_remote_backend_with_client_and_callback() -> (
    Client<shared::SinkMessage>,
    Callback<shared::SinkCallbackMessage>,
) {
    let (sender_for_client, recv_for_backend) = mpsc::channel();

    let (callback_from_backend, callback_to_client) = mpsc::channel();

    thread::spawn(move || remotebridge::run_forever(recv_for_backend, callback_from_backend));

    (
        Client::new(sender_for_client),
        Callback::new(callback_to_client),
    )
}
