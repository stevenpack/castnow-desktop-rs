use std::thread;
use std::thread::{JoinHandle};
use std::sync::mpsc::{Sender, Receiver};
use std::error::Error;
use castnow::{NodeModuleWrapper, Command, KeyCommand};
use state::State;
pub struct Processor {
}

impl Processor {
    pub fn new() -> Processor {
        Processor {}
    }

    pub fn start(&self, rx: Receiver<Command>, tx: Sender<State>) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut exit = false;
            while !exit {
                match rx.recv() {
                    Ok(cmd) => {
                        Self::process(&cmd);
                        //Example
                        tx.send(State::Error);
                    } 
                    Err(err) => {
                        //todo: If we're exiting, check that and don't try receive again so we don't end up with this error
                        println!("Error on recv {:?} {:?} {:?}", err, err.cause(), err.description());
                        exit = true;
                    }
                }
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