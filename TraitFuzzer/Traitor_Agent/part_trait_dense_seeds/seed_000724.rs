trait SerialPort {}

struct Example {
    member0: &'static [u8],
    member2: fn(&Box<dyn SerialPort>),
}

fn function1(_: &Box<dyn SerialPort>) {}

const EXAMPLE_MAP: &[Example] = &[
    Example {
        member0: "0".as_bytes(),
        member2: function1,
    },
    Example {
        member0: "0".as_bytes(),
        member2: function1,
    },
];

trait PortProcessor {
    fn process(&self, port: &Box<dyn SerialPort>);
}

impl PortProcessor for Example {
    fn process(&self, port: &Box<dyn SerialPort>) {
        (self.member2)(port);
    }
}

fn main() {
    let port = todo!();

    for example_member in EXAMPLE_MAP {
        example_member.process(&port);
    }
}