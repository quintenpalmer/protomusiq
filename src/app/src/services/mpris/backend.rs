use std::process;
use std::sync::mpsc;
use std::thread;

use mpris_player::OrgMprisMediaPlayer2;

use crate::shared;

pub fn run_forever(
    gui_to_mpris_rx: mpsc::Receiver<shared::MprisMessage>,
    mpris_to_gui_callback: mpsc::Sender<shared::MprisCallbackMessage>,
) {
    println!("MPRIS:\tstarting to run forever...");

    let (glib_sender, glib_recv) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let main_context = glib::MainContext::default();
    let main_loop = glib::MainLoop::new(Some(&main_context), false);

    thread::spawn(move || relay_from_gui_to_glib(gui_to_mpris_rx, glib_sender));

    main_context.spawn(async move { handle_mpris_from_gui(glib_recv, mpris_to_gui_callback) });

    main_loop.run();
}

fn relay_from_gui_to_glib(
    gui_to_mpris_rx: mpsc::Receiver<shared::MprisMessage>,
    glib_sender: glib::Sender<shared::MprisMessage>,
) {
    println!("MPRIS:\trelay has started");
    loop {
        println!("MPRIS:\twaiting for new message");
        match gui_to_mpris_rx.recv() {
            Ok(msg) => {
                println!("MPRIS:\trelaying a message {:?}", msg);
                match glib_sender.send(msg) {
                    Ok(()) => (),
                    Err(e) => println!("MPRIS:\terror relaying message to gui: {:?}", e),
                }
            }
            Err(e) => {
                println!("MPRIS:\terror receiving message to relay: {:?}", e);
                break;
            }
        };
    }
    println!("MPRIS:\trelayer is shutting down")
}

fn handle_mpris_from_gui(
    glib_receiver: glib::Receiver<shared::MprisMessage>,
    mpris_to_gui_callback: mpsc::Sender<shared::MprisCallbackMessage>,
) {
    println!("MPRIS:\tstarting gui receiver...");

    let player = mpris_player::MprisPlayer::new(
        format!("musiq.instance{}", process::id()),
        "Musiq".to_string(),
        "musiq".to_string(),
    );

    set_all_controls(&player, true);

    let play_pause_callback = mpris_to_gui_callback.clone();
    player.connect_play_pause(move || handle_play_pause(&play_pause_callback));

    let play_callback = mpris_to_gui_callback.clone();
    player.connect_play(move || handle_play(&play_callback));

    let pause_callback = mpris_to_gui_callback.clone();
    player.connect_pause(move || handle_pause(&pause_callback));

    let prev_callback = mpris_to_gui_callback.clone();
    player.connect_previous(move || handle_prev(&prev_callback));

    let next_callback = mpris_to_gui_callback.clone();
    player.connect_next(move || handle_next(&next_callback));

    println!("MPRIS:\tstarting to listen...");

    let forever_loop_context = glib::MainContext::default();

    glib_receiver.attach(
        Some(&forever_loop_context),
        move |msg: shared::MprisMessage| {
            println!("MPRIS:\tI got a message to process: {:?}", msg);
            match msg {
                shared::MprisMessage::SetMetadata(artist, title) => {
                    let mut metadata = mpris_player::Metadata::new();
                    metadata.artist = Some(vec![artist]);
                    metadata.title = Some(title);
                    println!("MPRIS:\tabout to set metadata");
                    player.set_metadata(metadata);
                    println!("MPRIS:\tset metadata, about to set playback status");
                    player.set_playback_status(mpris_player::PlaybackStatus::Playing);
                    println!("MPRIS:\tset playback status");
                    glib::Continue(true)
                }
                shared::MprisMessage::SetStopped => {
                    player.set_playback_status(mpris_player::PlaybackStatus::Stopped);
                    glib::Continue(true)
                }
                shared::MprisMessage::SetPlaying => {
                    player.set_playback_status(mpris_player::PlaybackStatus::Playing);
                    glib::Continue(true)
                }
                shared::MprisMessage::SetPaused => {
                    player.set_playback_status(mpris_player::PlaybackStatus::Paused);
                    glib::Continue(true)
                }
                shared::MprisMessage::Close => {
                    player.quit().unwrap();
                    glib::Continue(false)
                }
            }
        },
    );
}

fn set_all_controls(player: &mpris_player::MprisPlayer, to_set: bool) {
    player.set_can_control(to_set);
    player.set_can_play(to_set);
    player.set_can_pause(to_set);
    player.set_can_go_next(to_set);
    player.set_can_go_previous(to_set);
}

fn handle_play_pause(mpris_to_gui_callback: &mpsc::Sender<shared::MprisCallbackMessage>) {
    println!("MPRIS:\tsending play-pause to gui...");
    mpris_to_gui_callback
        .send(shared::MprisCallbackMessage::PlayPause)
        .unwrap()
}

fn handle_play(mpris_to_gui_callback: &mpsc::Sender<shared::MprisCallbackMessage>) {
    println!("MPRIS:\tsending play to gui...");
    mpris_to_gui_callback
        .send(shared::MprisCallbackMessage::Play)
        .unwrap()
}

fn handle_pause(mpris_to_gui_callback: &mpsc::Sender<shared::MprisCallbackMessage>) {
    println!("MPRIS:\tsending pause to gui...");
    mpris_to_gui_callback
        .send(shared::MprisCallbackMessage::Pause)
        .unwrap()
}

fn handle_prev(mpris_to_gui_callback: &mpsc::Sender<shared::MprisCallbackMessage>) {
    println!("MPRIS:\tsending prev to gui...");
    mpris_to_gui_callback
        .send(shared::MprisCallbackMessage::Prev)
        .unwrap()
}

fn handle_next(mpris_to_gui_callback: &mpsc::Sender<shared::MprisCallbackMessage>) {
    println!("MPRIS:\tsending next to gui...");
    mpris_to_gui_callback
        .send(shared::MprisCallbackMessage::Next)
        .unwrap()
}
