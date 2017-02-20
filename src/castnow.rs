use std::vec::Vec;

pub enum Command {
    Launch(String),
    KeyCommand(String),
    Quit,
}

pub enum KeyCommand {
    Pause,
    Mute,
    Stop
}

impl KeyCommand {
    fn get_key(cmd: KeyCommand) -> &'static str {
        match cmd {
            KeyCommand::Pause => "space",
            KeyCommand::Mute => "m",
            KeyCommand::Stop => "s"
        }
    }
}

struct CastNow {
}

impl CastNow {

    pub fn new() -> CastNow {

        let s = KeyCommand::get_key(KeyCommand::Mute);
        println!("action {:?}", s);

        return CastNow{
        };
    }

    pub fn execute(&self, command: Command) {
        match command {
            Command::Launch(path) => println!("Launch: {:?}", path),
            Command::Quit => println!("{:?}", "Quit"),
            Command::KeyCommand(cmd) => println!("KeyCommand: {:?}", cmd)
        }
    }
}