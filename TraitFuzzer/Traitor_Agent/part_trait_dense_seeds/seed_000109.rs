#![allow(dead_code)]

pub trait EventLoop {
    fn dummy(&self) { }
}

pub trait EventLoopExt: EventLoop {
    fn dummy_ext(&self);
}

impl<T: ?Sized + EventLoop> EventLoopExt for T {
    fn dummy_ext(&self) {
        self.dummy()
    }
}

pub struct UvEventLoop {
    uvio: isize
}

impl UvEventLoop {
    pub fn new() -> UvEventLoop {
        UvEventLoop {
            uvio: 0
        }
    }
}

impl EventLoop for UvEventLoop {
}

pub struct Scheduler {
    event_loop: Box<dyn EventLoop+'static>,
}

impl Scheduler {

    pub fn new(event_loop: Box<dyn EventLoop+'static>) -> Scheduler {
        Scheduler {
            event_loop: event_loop,
        }
    }

    pub fn dummy_event(&self) {
        self.event_loop.dummy_ext();
    }
}

pub fn main() {
    let sched = Scheduler::new(Box::new(UvEventLoop::new()) as Box<dyn EventLoop>);
    sched.dummy_event();
}