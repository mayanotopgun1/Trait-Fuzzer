use std::env;
use std::process::Command;
use std::thread;
use std::time::Duration;

trait CommandRunner {
    fn run_sleep(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn run_quickly(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

impl CommandRunner for Command {
    fn run_sleep(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut child = self.arg("sleep").spawn()?;
        thread::sleep(Duration::new(1_000, 0));
        child.kill().unwrap();
        child.wait()?;
        Ok(())
    }

    fn run_quickly(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut child = self.arg("return-quickly").spawn()?;
        loop {
            match child.try_wait()? {
                Some(status) => {
                    if status.success() {
                        break;
                    } else {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "child process did not exit successfully",
                        )));
                    }
                }
                None => thread::sleep(Duration::from_millis(1)),
            }
        }
        Ok(())
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 1 {
        match &args[1][..] {
            "sleep" => thread::sleep(Duration::new(1_000, 0)),
            _ => {}
        }
        return
    }

    let mut me = Command::new(env::current_exe().unwrap());
    me.run_sleep().unwrap();

    me = Command::new(env::current_exe().unwrap());
    me.run_quickly().unwrap();
}