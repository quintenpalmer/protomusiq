use std::sync::mpsc;

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
