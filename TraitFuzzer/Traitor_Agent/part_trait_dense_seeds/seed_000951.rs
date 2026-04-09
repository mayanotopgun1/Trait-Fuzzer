#![feature(type_alias_impl_trait)]

enum StdioContainer {
    CreatePipe(bool)
}

trait IoAccess {
    type IoType;
    fn get_io(&self) -> &[Self::IoType];
}

trait IoDebug: IoAccess<IoType = StdioContainer> {
    fn debug_io(&self) {
        for io in self.get_io() {
            match io {
                StdioContainer::CreatePipe(b) => println!("CreatePipe({})", b),
            }
        }
    }
}

impl<T: IoAccess<IoType = StdioContainer>> IoDebug for T {}

struct Test<'a> {
    args: &'a [String],
    io: &'a [StdioContainer]
}

impl<'a> IoAccess for Test<'a> {
    type IoType = StdioContainer;
    fn get_io(&self) -> &[Self::IoType] {
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