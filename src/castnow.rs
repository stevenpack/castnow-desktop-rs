use launcher::Launcher;

pub struct Command {
    pub key: KeyCommand,
    pub state: String
}

impl Command {
    pub fn new(key: KeyCommand) -> Command {
        Self::new_with_state(key, String::default())
    }

    pub fn new_with_state(key: KeyCommand, state: String) -> Command {
        Command {
            key: key,
            state: state
        }
    }
}

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

    pub fn execute(&self, command: &Command) {
        self.launcher.execute(&command.key);
    }
}