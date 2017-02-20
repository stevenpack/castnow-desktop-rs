
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
}

impl CastNow {

    pub fn new() -> CastNow {
        
        return CastNow{
        };
    }

    pub fn execute(&self, command: &KeyCommand) {
        match command {
            command => println!("{:?}", KeyCommand::get_key(command))
        }
    }
}