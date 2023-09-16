use std::path;
use std::sync::mpsc;

use crate::model;

#[derive(Debug, Clone)]
pub enum GUIToBackendMessage {
    ToSink(SinkMessage),
}

#[derive(Debug, Clone)]
pub enum BackendToGUIMessage {
    SinkReports(SinkCallbackMessage),
    MprisReports(MprisCallbackMessage),
}

#[derive(Debug, Clone)]
pub enum SinkMessage {
    PlayButton,
    PauseButton,
    LoadSong(path::PathBuf, f32),
    SetVolume(f32),
    Close,
}

#[derive(Debug, Clone)]
pub enum SinkCallbackMessage {
    Playing,
    Paused,
    SecondElapsed,
    SongEnded,
}

#[derive(Debug, Clone)]
pub enum MprisMessage {
    SetMetadata(String, String),
    SetPlaying,
    SetPaused,
    SetStopped,
    Close,
}

#[derive(Debug, Clone)]
pub enum MprisCallbackMessage {
    PlayPause,
    Play,
    Pause,
    Prev,
    Next,
}

#[derive(Debug, Clone)]
pub enum TrackerMessage {
    SongStarted(model::AugmentedTrack),
}

#[derive(Debug, Clone)]
pub struct Client<T> {
    sender: mpsc::Sender<T>,
}

impl<T> Client<T> {
    pub fn new(sender: mpsc::Sender<T>) -> Self {
        Client { sender: sender }
    }

    pub fn send(&self, message: T) -> Result<(), mpsc::SendError<T>> {
        self.sender.send(message)
    }
}

pub struct Callback<T> {
    receiver: mpsc::Receiver<T>,
}

impl<T> Callback<T> {
    pub fn new(receiver: mpsc::Receiver<T>) -> Self {
        Callback { receiver: receiver }
    }

    pub fn recv(&self) -> Result<T, mpsc::RecvError> {
        self.receiver.recv()
    }

    pub fn try_recv(&self) -> Result<T, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }
}
