#![feature(impl_trait_in_assoc_type)]
#![allow(dead_code, unused)]

use std::task::Poll;

struct K;
struct E;

trait ResultExt {
    fn as_poll(&self) -> impl Fn() -> Poll<Result<(), E>>;
    fn as_poll_option(&self) -> impl Fn() -> Poll<Option<Result<(), E>>>;
}

impl ResultExt for () {
    fn as_poll(&self) -> impl Fn() -> Poll<Result<(), E>> {
        move || {
            let _ = Ok::<(), E>(())?;
            let _: Poll<()> = Poll::Ready::<Result<(), E>>(Ok(()))?;
            let _: Poll<Option<()>> = Poll::Ready::<Option<Result<(), E>>>(None)?;

            Poll::Ready(Ok(()))
        }
    }

    fn as_poll_option(&self) -> impl Fn() -> Poll<Option<Result<(), E>>> {
        move || {
            let _ = Ok::<(), E>(())?;
            let _: Poll<()> = Poll::Ready::<Result<(), E>>(Ok(()))?;
            let _: Poll<Option<()>> = Poll::Ready::<Option<Result<(), E>>>(None)?;

            Poll::Ready(Some(Ok(())))
        }
    }
}

fn as_result() -> Result<(), E> {
    let closure = ().as_poll();
    match closure() {
        Poll::Ready(result) => result,
        _ => Err(E),
    }
}

fn as_poll_result() -> impl Fn() -> Poll<Result<(), E>> {
    move || {
        let closure = ().as_poll();
        closure()
    }
}

fn as_poll_option_result() -> impl Fn() -> Poll<Option<Result<(), E>>> {
    move || {
        let closure = ().as_poll_option();
        closure()
    }
}

fn main() {}