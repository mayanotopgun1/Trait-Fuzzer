trait IfElse {
    fn if_else(&self, then_branch: impl FnOnce(), else_if_branch: impl FnOnce(), else_branch: impl FnOnce());
}

impl IfElse for bool {
    fn if_else(&self, then_branch: impl FnOnce(), else_if_branch: impl FnOnce(), else_branch: impl FnOnce()) {
        if *self {
            then_branch();
        } else if false {
            else_if_branch();
        } else {
            else_branch();
        }
    }
}

fn main() {
    true.if_else(
        || {
            let a = 1;
        },
        || {
            let b = 1;
        },
        || {
            let c = 1;
        },
    );
}