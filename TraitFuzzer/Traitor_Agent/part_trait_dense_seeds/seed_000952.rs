#![feature(type_alias_impl_trait)]

enum StdioContainer {
    CreatePipe(bool)
}

trait IoAccess {
    type IoType<'a> where Self: 'a;
    fn get_io(&self) -> &[Self::IoType<'static>];
}

trait IoDebug: for<'a> IoAccess<IoType<'a> = StdioContainer> + 'static {
    fn debug_io(&self) {
        for io in self.get_io() {
            match io {
                StdioContainer::CreatePipe(b) => println!("CreatePipe({})", b),
            }
        }
    }
}

impl<T: for<'a> IoAccess<IoType<'a> = StdioContainer> + 'static> IoDebug for T {}

struct Test<'a> {
    args: &'a [String],
    io: &'a [StdioContainer]
}

impl<'a> IoAccess for Test<'a> {
    type IoType<'b> = StdioContainer where Self: 'b;
    fn get_io(&self) -> &[Self::IoType<'a>] {
        self.io
    }
}

pub fn main() {
    let test = Test {
        args: &[],
        io: &[StdioContainer::CreatePipe(true)]
    };

    test.debug_io();
}