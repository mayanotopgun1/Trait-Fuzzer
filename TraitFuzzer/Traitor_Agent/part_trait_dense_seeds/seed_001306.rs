trait MatchPattern {
    fn matches(&self, pattern: &(i32, i32, i32));
}

impl MatchPattern for (i32, i32, i32) {
    fn matches(&self, pattern: &(i32, i32, i32)) {
        match self {
            pat => {}
        }
    }
}

fn main() {
    let tuple = (0, 1, 2);
    tuple.matches(&(0, 1, 2));
}