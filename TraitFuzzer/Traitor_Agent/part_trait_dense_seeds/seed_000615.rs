#![feature(type_alias_impl_trait)]

#[allow(dead_code)]
fn macros() {
    macro_rules! foo {
        ($p:pat, $e:expr, $b:block) => {{
            while let $p = $e $b


        }}
    }
    macro_rules! bar {
        ($p:pat, $e:expr, $b:block) => {{
            foo!($p, $e, $b)
        }}
    }

    foo!(_a, 1, {
        println!("irrefutable pattern");
    });
    bar!(_a, 1, {
        println!("irrefutable pattern");
    });
}

pub trait WhileLet {
    fn while_let(&self);
}

impl WhileLet for i32 {
    fn while_let(&self) {
        let mut x = *self;
        while let _a = x {
            println!("irrefutable pattern");
            break;
        }
    }
}

pub fn main() {
    let x = 1;
    x.while_let();
}