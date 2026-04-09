#![deny(unreachable_patterns)]

mod inner {
    #[derive(PartialEq, Eq)]
    pub struct PrivateField {
        pub x: bool,
        y: bool,
    }

    impl PrivateField {
        pub fn is_foo(&self) -> bool {
            self.x && self.y
        }

        pub fn is_bar(&self) -> bool {
            self.x && !self.y
        }
    }

    pub const FOO: PrivateField = PrivateField { x: true, y: true };
    pub const BAR: PrivateField = PrivateField { x: true, y: false };
}
use inner::*;

fn main() {
    match &FOO {
        pf if pf.is_foo() => {}
        pf if pf.is_bar() => {}
        _ => {}
    }

    match &FOO {
        pf if pf.is_foo() => {}
        PrivateField { x: true, .. } => {}
        _ => {}
    }
}