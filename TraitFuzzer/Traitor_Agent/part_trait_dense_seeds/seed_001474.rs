macro_rules! homura {
    (#[$x:meta]) => ()
}

trait HomuraMeta {}
impl HomuraMeta for () {}

fn main() { }