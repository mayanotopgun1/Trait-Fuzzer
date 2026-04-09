#![warn(unused_must_use)]

#[cfg_attr(true, deprecated, must_use)]
#[derive(Debug)]
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
    let _ = <MustUseDeprecated as MustUseTrait>::new();
}