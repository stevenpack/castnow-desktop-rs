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
    Pause,
    Mute,
    Stop,
    Load
}

impl KeyCommand {
    pub fn get_key(key: &KeyCommand) -> &'static str {
        match key {
            &KeyCommand::Pause => "space",
            &KeyCommand::Mute => "m",
            &KeyCommand::Stop => "s",
            _ => "no key assigned"
        }
    }
}

pub struct NodeModuleWrapper {
    launcher: Launcher
}

impl NodeModuleWrapper {

    pub fn new() -> NodeModuleWrapper {
        return NodeModuleWrapper{
            launcher: Launcher::new()
        };
    }

    pub fn load(&self, file: &String) {
        self.launcher.load(file);
    }

    pub fn execute(&self, command: &Command) -> Result<(),Error> {
        self.launcher.execute(&command.key)
    }
}