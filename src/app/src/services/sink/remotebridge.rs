use std::sync::mpsc;

use crate::shared;

pub fn run_forever(
    rx: mpsc::Receiver<shared::SinkMessage>,
    _callback: mpsc::Sender<shared::SinkCallbackMessage>,
) {
    println!("SINK:\tstarting to relay...");

    // TODO: Actually relay using a web layer

    loop {
        let keep_running = relay_msg(rx.try_recv());
        if !keep_running {
            break;
        }
    }

    println!("SINK:\tdone relaying");
}

fn relay_msg(maybe_msg: Result<shared::SinkMessage, mpsc::TryRecvError>) -> bool {
    match maybe_msg {
        Ok(msg) => match msg {
            shared::SinkMessage::PlayButton => {
                ureq::post("http://localhost:5269/play").call().unwrap();
                true
            }
            shared::SinkMessage::PauseButton => {
                ureq::post("http://localhost:5269/pause").call().unwrap();
                true
            }
            shared::SinkMessage::LoadSong(full_path_for_music_file, volume_to_set) => {
                let path_str = full_path_for_music_file
                    .into_os_string()
                    .to_string_lossy()
                    .to_string();
                let volume_str = format!("{}", volume_to_set);
                let payload = [path_str, volume_str].join("\n");

                ureq::post("http://localhost:5269/load")
                    .send_string(payload.as_str())
                    .unwrap();
                true
            }
            shared::SinkMessage::SetVolume(volume_to_set) => {
                let payload = format!("{}", volume_to_set);

                ureq::post("http://localhost:5269/volume")
                    .send_string(payload.as_str())
                    .unwrap();
                true
            }
            shared::SinkMessage::Close => {
                ureq::post("http://localhost:5269/close").call().unwrap();
                false
            }
        },
        Err(mpsc::TryRecvError::Empty) => true,
        Err(mpsc::TryRecvError::Disconnected) => false,
    }
}
