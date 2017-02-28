use std::fmt;
use castnow::KeyCommand;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Initial,
    Loading,
    Loaded,
    Stopping,
    Stopped,
    Playing,
    Paused,
    Error
}

impl State {
    pub fn next(current: &State, cmd: &KeyCommand) -> State {
        let new_state = match *current {
            State::Initial => {
                match *cmd {
                    KeyCommand::Load => State::Loading,
                    KeyCommand::Mute => State::Initial,
                    KeyCommand::Pause => State::Initial,
                    KeyCommand::Stop => State::Initial
                }
            },
            State::Loading => {
                match *cmd {
                    KeyCommand::Load => State::Loading,
                    KeyCommand::Mute => State::Loading,
                    KeyCommand::Pause => State::Loading,
                    KeyCommand::Stop => State::Stopping,                    
                }
            },
            State::Loaded => {
                match *cmd {
                    KeyCommand::Load => State::Loading,
                    KeyCommand::Mute => State::Loaded,
                    KeyCommand::Pause => State::Loaded,
                    KeyCommand::Stop => State::Loaded,
                }
            },
            State::Paused => {
                match *cmd {
                    KeyCommand::Load => State::Loading,
                    KeyCommand::Mute => State::Paused,
                    KeyCommand::Pause => State::Paused,
                    KeyCommand::Stop => State::Stopped,
                }
            },
            State::Playing => {
                match *cmd {
                    KeyCommand::Load => State::Loading,
                    KeyCommand::Mute => State::Playing,
                    KeyCommand::Pause => State::Paused,
                    KeyCommand::Stop => State::Stopped
                }
            },
            State::Stopping => {
                match *cmd {
                    KeyCommand::Load => State::Loading,
                    KeyCommand::Mute => State::Stopping,
                    KeyCommand::Pause => State::Stopping,
                    KeyCommand::Stop => State::Stopping
                }
            }
            State::Stopped => {
                match *cmd { 
                    KeyCommand::Load => State::Loading,
                    KeyCommand::Mute => State::Stopped,
                    KeyCommand::Pause => State::Stopped,
                    KeyCommand::Stop => State::Stopped
                }
            },
            State::Error => {
                match *cmd {
                    KeyCommand::Load => State::Loading,
                    KeyCommand::Mute => State::Error,
                    KeyCommand::Pause => State::Paused,
                    KeyCommand::Stop => State::Stopped
                }
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