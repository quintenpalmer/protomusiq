use std::sync::mpsc;
use std::thread;
use std::time;

use crate::shared;

use super::remotehandler;

pub fn run_forever(
    rx: mpsc::Receiver<shared::SinkMessage>,
    callback: mpsc::Sender<shared::SinkCallbackMessage>,
) {
    println!("SINK:\tstarting to relay...");

    // TODO: Actually relay using a web layer

    let callback_server = remotehandler::CallbackServer::new();

    loop {
        let keep_running = relay_msg(rx.try_recv());
        match callback_server.try_recv() {
            Some(Ok(msg)) => callback.send(msg).unwrap(),
            Some(Err(e)) => {
                println!("error on remote bridge: {:?}", e);
                break;
            }
            None => (),
        }

        if !keep_running {
            break;
        }

        thread::sleep(time::Duration::from_millis(50));
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
            shared::SinkMessage::LoadSong(full_path_for_music_file, next_thing, volume_to_set) => {
                let path_str = full_path_for_music_file
                    .into_os_string()
                    .to_string_lossy()
                    .to_string();
                let next_thing = match next_thing {
                    Some(shared::TrackPathOrPause::TrackPath(p)) => {
                        p.into_os_string().to_string_lossy().to_string()
                    }
                    Some(shared::TrackPathOrPause::Pause) => "pause".to_string(),
                    None => "none".to_string(),
                };

                let volume_str = format!("{}", volume_to_set);
                let payload = [path_str, next_thing, volume_str].join("\n");

                ureq::post("http://localhost:5269/load")
                    .send_string(payload.as_str())
                    .unwrap();
                true
            }
            shared::SinkMessage::LoadNextSong(next_thing, volume_to_set) => {
                let next_thing = match next_thing {
                    Some(shared::TrackPathOrPause::TrackPath(p)) => {
                        p.into_os_string().to_string_lossy().to_string()
                    }
                    Some(shared::TrackPathOrPause::Pause) => "pause".to_string(),
                    None => "none".to_string(),
                };

                let volume_str = format!("{}", volume_to_set);
                let payload = [next_thing, volume_str].join("\n");

                ureq::post("http://localhost:5269/loadnextsong")
                    .send_string(payload.as_str())
                    .unwrap();
                true
            }
            shared::SinkMessage::SetNextSong(full_path_for_music_file) => {
                let payload = match full_path_for_music_file {
                    shared::TrackPathOrPause::TrackPath(t) => {
                        t.into_os_string().to_string_lossy().to_string()
                    }
                    shared::TrackPathOrPause::Pause => "pause".to_string(),
                };

                ureq::post("http://localhost:5269/setnextsong")
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
