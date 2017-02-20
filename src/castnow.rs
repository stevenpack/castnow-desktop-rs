use launcher::Launcher;

pub enum KeyCommand {
    Pause,
    Mute,
    Stop
}

impl KeyCommand {
    pub fn get_key(cmd: &KeyCommand) -> &'static str {
        match cmd {
            &KeyCommand::Pause => "space",
            &KeyCommand::Mute => "m",
            &KeyCommand::Stop => "s",
        }
    }
}

pub struct CastNow {
    launcher: Launcher
}

impl CastNow {

    pub fn new() -> CastNow {
        return CastNow{
            launcher: Launcher::new()
        };
    }

    pub fn load(&self, file: String) {
        self.launcher.load(file);
    }

    pub fn execute(&self, command: &KeyCommand) {
        self.launcher.execute(command);
    }
}