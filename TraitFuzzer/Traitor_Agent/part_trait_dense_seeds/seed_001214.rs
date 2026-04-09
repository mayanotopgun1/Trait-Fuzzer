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

pub fn foo() -> impl core::fmt::Debug {
    if let Some(x) = None::<UnOp> {
        x.match_not();
    } else {
        ()
    }
}

fn main() {}