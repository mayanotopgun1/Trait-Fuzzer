#![feature(linkage)]

trait Linkage {
    fn is_linked(&self) -> bool;
}

impl Linkage for bool {
    fn is_linked(&self) -> bool { *self }
}

#[linkage = "external"]
pub static TEST2: bool = true;

#[linkage = "internal"]
pub static TEST3: bool = true;

#[cfg(not(target_env = "msvc"))]
#[linkage = "linkonce"]
pub static TEST4: bool = true;

#[cfg(not(target_env = "msvc"))]
#[linkage = "linkonce_odr"]
pub static TEST5: bool = true;

#[linkage = "weak"]
pub static TEST7: bool = true;

#[linkage = "weak_odr"]
pub static TEST8: bool = true;

fn main() {
    let _ = (&TEST2 as &dyn Linkage).is_linked();
    let _ = (&TEST3 as &dyn Linkage).is_linked();
    #[cfg(not(target_env = "msvc"))]
    let _ = (&TEST4 as &dyn Linkage).is_linked();
    #[cfg(not(target_env = "msvc"))]
    let _ = (&TEST5 as &dyn Linkage).is_linked();
    let _ = (&TEST7 as &dyn Linkage).is_linked();
    let _ = (&TEST8 as &dyn Linkage).is_linked();
}