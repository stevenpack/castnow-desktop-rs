use std::io::Write;
use std::process::{Command, Child, Stdio};
use std::result::Result;
use castnow::KeyCommand;

pub struct Launcher {
    child: Option<Child>
}

impl Launcher {
    pub fn new() -> Launcher {
        return Launcher {
            child: None
        };
    }

    pub fn launch(&mut self) {
        let res_spawn = Command::new("castnow")
                        .arg("/home/steve/Downloads/SampleVideo_1280x720_5mb.mp4")
                        .arg("--exit")
                        .spawn();
        match res_spawn {
            Ok(child) => {
                println!("PID {:?}", child.id());
                self.child = Some(child);
            },
            Err(e) => println!("Spawn failed {:?}", e)
        }        
    }

    pub fn execute(&self, cmd: &KeyCommand) {
        let key = KeyCommand::get_key(cmd);
        let shell_cmd = format!("castnow --command {0} --quiet --exit", key);
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

 // pub fn pause(&mut self) -> Result<u32, &'static str> {
    //     if self.child.is_none() {
    //         return Err("No child process")
    //     }
    //     if let Some(ref mut child) = self.child {
    //         if let Some(ref mut stdin) = child.stdin {
    //             let b = "s".as_bytes();
    //             if let Some(bytes_written) = stdin.write(b).ok(){
    //                 stdin.flush().expect("Failed to flush");
    //                 println!("Sent {:?}", bytes_written);
    //             }
    //             return Ok(1 as u32);
    //         }
    //     }
    //     unreachable!("No child or stdin should have been picked up.");
    // }