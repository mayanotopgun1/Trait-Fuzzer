#[cfg(target_family = "windows")]
trait MainRunner {
    fn run(&self);
}

#[cfg(target_family = "windows")]
impl MainRunner for () {
    fn run(&self) {}
}

#[cfg(target_family = "unix")]
trait MainRunner {
    fn run(&self);
}

#[cfg(target_family = "unix")]
impl MainRunner for () {
    fn run(&self) {}
}

#[cfg(all(target_family = "wasm", not(target_os = "emscripten")))]
trait MainRunner {
    fn run(&self);
}

#[cfg(all(target_family = "wasm", not(target_os = "emscripten")))]
impl MainRunner for () {
    fn run(&self) {}
}

#[cfg(target_family = "windows")]
pub fn main() {
    let runner: Box<dyn MainRunner> = Box::new(());
    runner.run();
}

#[cfg(target_family = "unix")]
pub fn main() {
    let runner: Box<dyn MainRunner> = Box::new(());
    runner.run();
}

#[cfg(all(target_family = "wasm", not(target_os = "emscripten")))]
pub fn main() {
    let runner: Box<dyn MainRunner> = Box::new(());
    runner.run();
}