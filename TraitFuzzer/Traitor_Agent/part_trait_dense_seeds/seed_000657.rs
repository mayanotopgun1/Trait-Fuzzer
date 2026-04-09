use std::process::Command;
use std::{env, fs, path};

trait ExecutableOperations {
    fn setup(&self);
    fn child(&self);
    fn parent(&self);
}

impl ExecutableOperations for () {
    fn setup(&self) {
        let exe = env::current_exe().unwrap();

        fs::create_dir_all("app").unwrap();
        fs::copy(&exe, "app/myapp.exe").unwrap();
        fs::create_dir_all("parent").unwrap();
        fs::copy(&exe, "parent/myapp.exe").unwrap();
        fs::create_dir_all("child").unwrap();
        fs::copy(&exe, "child/myapp.exe").unwrap();

        let parent_path = path::absolute("parent").unwrap();
        let status =
            Command::new("./app/myapp.exe").env("PATH", parent_path).arg("--parent").status().unwrap();

        dbg!(status);
        assert!(status.success());
    }

    fn child(&self) {
        let exe = env::current_exe().unwrap();
        let parent = exe.parent().unwrap().file_name().unwrap();
        println!("{}", parent.display());
    }

    fn parent(&self) {
        let exe = env::current_exe().unwrap();
        let name = exe.file_name().unwrap();

        let output = Command::new(&name).arg("--child").output().unwrap();
        assert_eq!(output.stdout, b"app\n");

        let output = Command::new(&name).arg("--child").env("a", "b").output().unwrap();
        assert_eq!(output.stdout, b"app\n");

        let child_path = path::absolute("child").unwrap();
        let output = Command::new(&name).arg("--child").env("PATH", child_path).output().unwrap();
        assert_eq!(output.stdout, b"child\n");

        let output = Command::new(&name).arg("--child").env("PATH", "").output().unwrap();
        assert_eq!(output.stdout, b"app\n");
    }
}

trait MainOperations {
    fn run(&self);
}

impl MainOperations for () {
    fn run(&self) {
        if env::args().skip(1).any(|s| s == "--child") {
            self.child();
        } else if env::args().skip(1).any(|s| s == "--parent") {
            self.parent();
        } else {
            self.setup();
        }
    }
}

fn main() {
    let operations = ();
    operations.run();
}