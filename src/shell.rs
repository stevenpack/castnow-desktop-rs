use std::process::{Command};
use castnow::KeyCommand;

pub struct Launcher {
}

impl Launcher {
    pub fn new() -> Launcher {
        return Launcher {
            //child: None
        };
    }

    pub fn load(&self, file_path: &String) {
        let shell_cmd = format!("castnow '{0}' --quiet --exit", file_path);
        self.execute_command(shell_cmd);
    }

    pub fn execute(&self, cmd: &KeyCommand) {
        let key = KeyCommand::get_key(cmd);
        let shell_cmd = format!("castnow --command {0} --quiet --exit", key);
        self.execute_command(shell_cmd);            
    }

    pub fn execute_command(&self, shell_cmd: String) {
        println!("executing: sh -c {}", shell_cmd);
        let res_spawn = Command::new("sh")
                                .arg("-c")
                                .arg(shell_cmd)
                                .spawn();

        match res_spawn {
            Ok(child) => println!("PID {:?}", child.id()),
            Err(e) => println!("Spawn failed {:?}", e)
        }   
    }
}