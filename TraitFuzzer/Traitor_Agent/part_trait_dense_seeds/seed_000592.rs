trait MainRunner<'a> {
    type Output;
    fn run(&'a self) -> Self::Output;
}

trait MainRunnerExt<'a>: MainRunner<'a> {
    fn run_with_log(&'a self) where Self::Output: core::fmt::Debug {
        let result = self.run();
        println!("Result: {:?}", result);
    }
}

impl<'a, T: ?Sized> MainRunnerExt<'a> for T where T: MainRunner<'a> {}

impl<'a> MainRunner<'a> for () {
    type Output = ();
    fn run(&'a self) -> Self::Output {}
}

fn main() {
    let runner: &dyn MainRunner<'static, Output = ()> = &();
    runner.run_with_log();
}