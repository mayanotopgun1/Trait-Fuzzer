#![crate_name="numeric"]
#![crate_type = "lib"]

pub trait Trig<T> {
    fn sin(&self) -> T;
}

pub trait TrigExt<T>: Trig<T> where T: core::ops::Add<Output = T> + Copy {
    fn double_sin(&self) -> T {
        let x = self.sin();
        x + x
    }
}

impl<T, R> TrigExt<R> for T where T: Trig<R>, R: core::ops::Add<Output = R> + Copy {}

pub fn sin<T:Trig<R>, R: std::fmt::Debug>(theta: &T) -> impl core::fmt::Debug { theta.sin() }

pub trait Angle<T>: Trig<T> {}