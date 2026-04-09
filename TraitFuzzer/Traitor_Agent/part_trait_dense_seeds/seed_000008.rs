use std::future::Future;

pub trait Service {
    type Response;
    type Future: Future<Output = Self::Response>;
}

trait ServiceExt<R, F>: Service<Response = R, Future = F> {}
impl<S, R, F> ServiceExt<R, F> for S where S: Service<Response = R, Future = F> {}

pub trait A1: Service<Response = i32> + ServiceExt<i32, Box<dyn Future<Output = i32>>> {}

trait A2Ext: A1 {
    fn foo(&self) {}
}
impl<T> A2Ext for T where T: A1 {}

pub trait A2: A2Ext {}
impl<S> A2 for S where S: A2Ext {}

pub trait B1: Service<Future = Box<dyn Future<Output = i32>>> + ServiceExt<i32, Box<dyn Future<Output = i32>>> {}

trait B2Ext: B1 {
    fn foo(&self) {}
}
impl<T> B2Ext for T where T: B1 {}

pub trait B2: B2Ext {}
impl<S> B2 for S where S: B2Ext {}

fn main() {
    let x: &dyn A2 = todo!();
    let x: &dyn B2 = todo!();
}