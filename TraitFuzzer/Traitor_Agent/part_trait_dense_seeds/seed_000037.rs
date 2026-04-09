macro_rules! define_exported { () => {
    #[macro_export]
    macro_rules! exported {
        () => ()
    }
}}

trait ExportMacro {
    fn export(&self);
}

impl ExportMacro for () {
    fn export(&self) {}
}

mod inner1 {
    use super::*;
    exported!();
}

mod inner2 {
    define_exported!();
}

fn main() {
    let _ = ().export();
}