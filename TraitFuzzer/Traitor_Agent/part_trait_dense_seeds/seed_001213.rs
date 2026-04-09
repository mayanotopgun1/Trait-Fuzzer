pub enum UnOp {
    Not(Vec<()>),
}

trait UnOpMatcher {
    fn match_not(&self);
}

impl UnOpMatcher for UnOp {
    fn match_not(&self) {
        if let UnOp::Not(_) = self {

        }
    }
}

pub fn foo() {
    if let Some(x) = None::<UnOp> {
        x.match_not();
    }
}

fn main() {}