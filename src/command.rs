use std::thread;
use std::thread::{JoinHandle};
use std::sync::mpsc::Receiver;
use castnow::{NodeModuleWrapper, Command, KeyCommand};

pub struct Processor {
}

impl Processor {
    pub fn new() -> Processor {
        Processor {}
    }

    pub fn start(&self, rx: Receiver<Command>) -> JoinHandle<()> {
        thread::spawn(move || {
            match rx.recv() {
                Ok(cmd) => Self::process(&cmd),
                Err(err) => println!("Error on recv {:?}", err)
            }
        })
    }

    fn process(cmd: &Command) {
        let node_module = NodeModuleWrapper::new();
        match cmd.key {
            KeyCommand::Load => node_module.load(&cmd.state),
            _ => node_module.execute(cmd)
        }
    }
}