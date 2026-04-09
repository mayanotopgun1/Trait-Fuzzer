#![feature(return_position_impl_trait_in_trait)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub struct Context<'tcx> {
    vec: &'tcx Vec<isize>
}

pub type Cmd<'a> = &'a isize;

pub trait DecodeItem<'b> {
    fn decode(&mut self, cmd: Cmd<'b>, ctx: &Context<'b>) -> Result<&'b isize, ()>;
}

impl<'a, 'tcx> DecodeItem<'a> for Box<dyn for<'b> FnMut(Cmd<'b>, &Context<'b>) -> Result<&'b isize, ()> + 'a> {
    fn decode(&mut self, cmd: Cmd<'a>, ctx: &Context<'a>) -> Result<&'a isize, ()> {
        (*self)(cmd, ctx)
    }
}

pub type DecodeInlinedItem<'a> = Box<dyn for<'b> DecodeItem<'b> + 'a>;

fn foo(mut d: DecodeInlinedItem) {
    let cmd = &0;
    let context = Context { vec: &vec![1, 2, 3] };
    match d.decode(cmd, &context) {
        Ok(value) => println!("Decoded value: {}", value),
        Err(_) => println!("Failed to decode"),
    }
}

fn main() { }