use std::fmt;
use castnow::KeyCommand;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Initial,
    Loading,
    Loaded,
    Playing,
    Stopping,
    Stopped,
    Pausing,
    Paused,
    Error
}

impl State {
    pub fn next(current: &State, success: bool) -> State {
        if !success {
            return State::Error;
        }

        let new_state = match *current {            
            State::Loading  => State::Loaded,
            State::Stopping => State::Stopped,
            State::Pausing => State::Paused,
            _ => {
                println!("Unexpected state transition {:?}", *current);
                State::Error
            }
        };
        println!("{:?} -> {:?}", current, new_state);
        new_state        
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}