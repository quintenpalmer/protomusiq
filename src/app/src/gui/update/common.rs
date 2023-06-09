use crate::shared;

pub fn tracker_sender<T: std::fmt::Debug>(
    tx: shared::Client<T>,
    message: T,
) -> MessageCommandSender<T> {
    MessageCommandSender::new("Tracker".to_string(), tx, message)
}

pub fn mpris_sender<T: std::fmt::Debug>(
    tx: shared::Client<T>,
    message: T,
) -> MessageCommandSender<T> {
    MessageCommandSender::new("Mpris".to_string(), tx, message)
}

pub fn sink_sender<T: std::fmt::Debug>(
    tx: shared::Client<T>,
    message: T,
) -> MessageCommandSender<T> {
    MessageCommandSender::new("Sink".to_string(), tx, message)
}

pub struct MessageCommandSender<T> {
    name: String,
    tx: shared::Client<T>,
    message: T,
}

impl<T: std::fmt::Debug> MessageCommandSender<T> {
    fn new(name: String, tx: shared::Client<T>, message: T) -> Self {
        MessageCommandSender {
            name: name,
            tx: tx,
            message: message,
        }
    }

    pub async fn send_message(self) -> Result<(), String> {
        println!("GUI:\t{}: payload is {:?}", self.name, self.message);
        match self.tx.send(self.message) {
            Ok(a) => {
                println!("GUI:\t{}: resp was {:?}", self.name, a);
                Ok(())
            }
            Err(e) => {
                println!("GUI:\t{}: err resp was {:?}", self.name, e);
                Err(format!("{:?}", e))
            }
        }
    }
}
