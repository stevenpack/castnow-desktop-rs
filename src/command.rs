use std::thread;
use std::thread::{JoinHandle};
use std::sync::mpsc::{Sender, Receiver};
use castnow::{Command, KeyCommand};
use state::State;
use shell::Launcher;
use std::error::Error;

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
                        let new_state = Self::process(&cmd);
                        match tx.send(new_state) {
                            Ok(_) => println!("Command processed. New state {:?}", new_state),
                            Err(e) => println!("Error processing command {:?}", e)
                        }
                    }, 
                    Err(err) => {
                        //todo: If we're exiting, check that and don't try receive again so we don't end up with this error
                        println!("Error on recv {:?} {:?} {:?}", err, err.cause(), err.description());
                        exit = true;
                    }
                }
            }
        })
    }

    fn process(cmd: &Command) -> State {
        let result = match cmd.key {
            KeyCommand::Load => Launcher::load(&cmd.state),
            _ => Launcher::execute(&cmd.key)
        };
        State::next(&cmd.current, result.is_ok())        
    }
}