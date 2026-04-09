#![feature(cfg_target_compact)]

trait TargetCheck {
    fn is_expected(&self) -> bool;
}

impl TargetCheck for () {
    #[cfg(target(os = "linux", arch = "arm"))]
    fn is_expected(&self) -> bool { true }

    #[cfg(not(target(os = "linux", arch = "arm")))]
    fn is_expected(&self) -> bool { false }
}

trait UnexpectedCheck {
    fn is_unexpected(&self) -> bool;
}

impl UnexpectedCheck for () {
    #[cfg(target(os = "linux", architecture = "arm"))]
    fn is_unexpected(&self) -> bool { true }

    #[cfg(not(target(os = "linux", architecture = "arm")))]
    fn is_unexpected(&self) -> bool { false }
}

trait Unexpected2Check {
    fn is_unexpected2(&self) -> bool;
}

impl Unexpected2Check for () {
    #[cfg(target(os = "windows", architecture = "arm"))]
    fn is_unexpected2(&self) -> bool { true }

    #[cfg(not(target(os = "windows", architecture = "arm")))]
    fn is_unexpected2(&self) -> bool { false }
}

fn main() {
    let _ = if <() as TargetCheck>::is_expected(&()) { () };
    let _ = if <() as UnexpectedCheck>::is_unexpected(&()) { () };
    let _ = if <() as Unexpected2Check>::is_unexpected2(&()) { () };
}