#![feature(generic_associated_types)]
#![feature(decl_macro)]

trait MacroExec {
    type Item<'a> where Self: 'a;
    fn exec(&self) -> Self::Item<'static>;
}

trait ExecExt: MacroExec + 'static {
    fn exec_with_message(&self) -> String where <Self as MacroExec>::Item<'static>: std::fmt::Display {
        format!("Result of execution: {}", self.exec())
    }
}

impl<T: MacroExec + 'static> ExecExt for T {}

struct NoOp;

impl MacroExec for NoOp {
    type Item<'a> = &'a str;
    fn exec(&self) -> Self::Item<'static> { "No operation executed" }
}

macro_rules! foo {
    () => {};
}

fn main() {
    let no_op = NoOp;
    println!("{}", no_op.exec_with_message());
}