use std::io::Error;
use std::process::{Command};
use castnow::KeyCommand;

pub struct Launcher {
}

impl Launcher {
    pub fn new() -> Launcher {
        Launcher {}
    }

    pub fn load(file_path: &String) -> Result<(),Error> {
        let shell_cmd = format!("castnow '{0}' --quiet --exit", file_path);
        Self::execute_command(shell_cmd)
    }

    pub fn execute(cmd: &KeyCommand) -> Result<(),Error> {
        let key = KeyCommand::get_key(cmd);
        let shell_cmd = format!("castnow --command {0} --quiet --exit", key);
        Self::execute_command(shell_cmd)        
    }

    pub fn execute_command(shell_cmd: String) -> Result<(),Error> {
        println!("executing: sh -c {}", shell_cmd);
        let res_spawn = Command::new("sh")
                                .arg("-c")
                                .arg(shell_cmd)
                                .spawn();

        match res_spawn {
            Ok(child) => {
                println!("PID {:?}", child.id());
                Ok(())
            }
            Err(e) => {
                println!("Spawn failed {:?}", e);
                Err(e)
            }
        }
           
    }
}