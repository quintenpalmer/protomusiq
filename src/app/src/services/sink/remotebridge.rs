use std::sync::mpsc;

use crate::shared;

use super::backend;

pub fn run_forever(
    rx: mpsc::Receiver<shared::SinkMessage>,
    callback: mpsc::Sender<shared::SinkCallbackMessage>,
) {
    println!("SINK:\tstarting to relay...");

    // TODO: Actually relay using a web layer

    backend::run_forever(rx, callback);

    println!("SINK:\tdone relaying");
}
