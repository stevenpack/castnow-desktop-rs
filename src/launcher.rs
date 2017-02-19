use std::io::Write;
use std::process::{Command, Child, Stdio};
use std::result::Result;
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
                        //.arg("-c")
                        //.arg("echo hello")
                        .arg("/home/steve/Downloads/SampleVideo_1280x720_5mb.mp4")
                        .arg("--exit")
                        //.stdin(Stdio::piped())
                        .spawn();
        match res_spawn {
            Ok(child) => {
                println!("PID {:?}", child.id());
                self.child = Some(child);
            },
            Err(e) => println!("Spawn failed {:?}", e)
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

    pub fn pause(&self) {
    let output = Command::new("sh")
                        .arg("-c")
                        //.arg("echo hello")
                        .arg("castnow --command space --quiet --exit")
                        .spawn();
                        //.output()
                        //.expect("failed to execute process");

    // let hello = String::from_utf8(output.stdout).unwrap();
    // println!("{:?}", hello);
}
}

// pub fn launch(cmd: &mut Command) -> Child {
//     let child = cmd.spawn().expect("failed to execute process");
//     println!("Launched");
//     return child;
// }

