#![feature(impl_trait_in_assoc_type)]

trait Mirror<'a> {
    type Assoc;
}

impl<'a, T: 'a> Mirror<'a> for T {
    type Assoc = &'a T;
}

trait MirrorExt<T>: Mirror<'static, Assoc = T> where T: 'static {
    fn check_static(&self) {
        is_static::<T>();
    }
}

impl<S, T> MirrorExt<T> for S where S: Mirror<'static, Assoc = T>, T: 'static {}

trait MirrorExtDefault<T>: MirrorExt<T> where T: std::default::Default + 'static {
    fn default_mirror() -> Self;
}

impl<S, T> MirrorExtDefault<T> for S where S: MirrorExt<T> + std::default::Default, T: std::default::Default + 'static {
    fn default_mirror() -> Self {
        Default::default()
    }
}

fn test<T: 'static + MirrorExtDefault<T>>() where <T as Mirror<'static>>::Assoc: 'static, T: std::default::Default {
    let _mirror = T::default_mirror();
    _mirror.check_static();
}

fn is_static<T>() where T: 'static {

}

fn main() {}