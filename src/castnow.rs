use std::io::Error;
use shell::Launcher;
use state::State;

#[derive(Debug)]
pub struct Command {
    pub key: KeyCommand,
    pub state: String,
    pub current: State
}

impl Command {
    pub fn new(key: KeyCommand, current: State) -> Command {
        Self::new_with_state(key, current, String::default())
    }

    pub fn new_with_state(key: KeyCommand, current: State, state: String) -> Command {
        Command {
            key: key,
            current: current,
            state: state
        }
    }
}

#[derive(Debug)]
pub enum KeyCommand {
    TogglePlayPause,
    ToggleMute,
    Stop,
    Load
}

impl KeyCommand {
    pub fn get_key(key: &KeyCommand) -> &'static str {
        match key {
            &KeyCommand::TogglePlayPause => "space",
            &KeyCommand::ToggleMute => "m",
            &KeyCommand::Stop => "s",
            _ => "no key assigned"
        }
    }
}