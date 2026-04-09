#![warn(unused_must_use)]

#[cfg_attr(true, deprecated, must_use)]
struct MustUseDeprecated {}

trait MustUseTrait {
    fn new() -> Self;
}

impl MustUseTrait for MustUseDeprecated {
    fn new() -> MustUseDeprecated {
        MustUseDeprecated {}
    }
}

fn main() {
    MustUseDeprecated::new();
}