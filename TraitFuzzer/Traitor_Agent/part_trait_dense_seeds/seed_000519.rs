use std::process::Command;
use std::env;
use std::collections::HashMap;

#[cfg(all(unix, not(target_os="android")))]
trait EnvCmd {
    fn env_cmd() -> Command;
}

#[cfg(target_os="android")]
trait EnvCmd {
    fn env_cmd() -> Command;
}

#[cfg(windows)]
trait EnvCmd {
    fn env_cmd() -> Command;
}

#[cfg(all(unix, not(target_os="android")))]
impl EnvCmd for () {
    fn env_cmd() -> Command {
        Command::new("env")
    }
}

#[cfg(target_os="android")]
impl EnvCmd for () {
    fn env_cmd() -> Command {
        let mut cmd = Command::new("/system/bin/sh");
        cmd.arg("-c").arg("set");
        cmd
    }
}

#[cfg(windows)]
impl EnvCmd for () {
    fn env_cmd() -> Command {
        let mut cmd = Command::new("cmd");
        cmd.arg("/c").arg("set");
        cmd
    }
}

fn main() {

    let old_env = env::var_os("RUN_TEST_NEW_ENV");

    env::set_var("RUN_TEST_NEW_ENV", "123");


    let filtered_env : HashMap<String, String> =
        env::vars().filter(|&(ref k, _)| k == "PATH").collect();

    let mut cmd = <() as EnvCmd>::env_cmd();
    cmd.env_clear();
    cmd.envs(&filtered_env);


    match old_env {
        None => env::remove_var("RUN_TEST_NEW_ENV"),
        Some(val) => env::set_var("RUN_TEST_NEW_ENV", &val)
    }

    let result = cmd.output().unwrap();
    let output = String::from_utf8_lossy(&result.stdout);

    assert!(!output.contains("RUN_TEST_NEW_ENV"),
            "found RUN_TEST_NEW_ENV inside of:\n\n{}", output);
}