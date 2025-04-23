pub mod utils;

use rumqttd::{Broker, Config};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

enum ControlMessage {
    Stop,
    Reload(Config),
}

pub struct EmbeddedBroker {
    control_tx: Sender<ControlMessage>,
    handle: Option<JoinHandle<()>>,
}

impl EmbeddedBroker {
    pub fn new(config: Config) -> Self {
        let (tx, rx): (Sender<ControlMessage>, Receiver<ControlMessage>) = mpsc::channel();
        let mut broker = Broker::new(config.clone());
        let handle = thread::spawn(move || {
            loop {
                broker.start().unwrap();

                match rx.try_recv() {
                    Ok(ControlMessage::Stop) => {
                        break;
                    }
                    Ok(ControlMessage::Reload(new_cfg)) => {
                        broker = Broker::new(new_cfg);
                        continue;
                    }
                    Err(_) => break, // no message, exit
                }
            }
        });

        EmbeddedBroker {
            control_tx: tx,
            handle: Some(handle),
        }
    }

    pub fn stop(&self) {
        let _ = self.control_tx.send(ControlMessage::Stop);
    }

    pub fn reload(&self, new_config: Config) {
        let _ = self.control_tx.send(ControlMessage::Reload(new_config));
    }
}
