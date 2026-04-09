trait SplitExt {
    fn split_collect(&self, delimiter: &str) -> Vec<&str>;
}

impl SplitExt for str {
    fn split_collect(&self, delimiter: &str) -> Vec<&str> {
        self.split(delimiter).collect()
    }
}

fn main() {
    let args = vec!["foobie", "asdf::asdf"];
    let arr: Vec<&str> = args[1].split_collect("::");
    assert_eq!(arr[0], "asdf");
    assert_eq!(arr[0], "asdf");
}