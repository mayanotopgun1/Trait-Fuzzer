#![feature(sanitize)]

trait Sanitize { fn sanitize_on(&self); }
trait Caller: Sanitize { fn caller(&self); }
trait Blocking: Caller { fn blocking(&self); }

impl Sanitize for () {
    fn sanitize_on(&self) {
        self.caller();
    }
}

impl Caller for () {
    fn caller(&self) {
        self.blocking();
    }
}

impl Blocking for () {
    #[sanitize(realtime = "blocking")]
    fn blocking(&self) {
        println!("blocking call not detected");
    }
}

fn main() {
    let _ = ().sanitize_on();
}