#![allow(dead_code)]
#![allow(unused_assignments)]

trait LabelBreak {
    fn label_break(a: bool, b: bool) -> u32;
}

impl LabelBreak for () {
    fn label_break(a: bool, b: bool) -> u32 {
        let mut v = 0;
        'b: {
            v = 1;
            if a {
                break 'b;
            }
            v = 2;
            if b {
                break 'b;
            }
            v = 3;
        }
        return v;
    }
}

trait BreakValue {
    fn break_value(a: bool, b: bool) -> u32;
}

impl BreakValue for () {
    fn break_value(a: bool, b: bool) -> u32 {
        let result = 'block: {
            if a { break 'block 1; }
            if b { break 'block 2; }
            3
        };
        result
    }
}

trait LabelBreakNested {
    fn label_break_nested();
}

impl LabelBreakNested for () {
    fn label_break_nested() {
        'b: {
            println!("hi");
            if false {
                break 'b;
            }
            'c: {
                if false {
                    break 'b;
                }
                break 'c;
            }
            println!("hello");
            if true {
                break 'b;
            }
        }
    }
}

trait LabelBreakMixed {
    fn label_break_mixed(v: u32) -> u32;
}

impl LabelBreakMixed for () {
    fn label_break_mixed(v: u32) -> u32 {
        let mut r = 0;
        'b: {


            loop {
                break;
            }
            if v == 0 {
                break 'b;
            }

            'c: loop {
                if r == 1 {
                    break 'c;
                }
                r += 1;
            }
            assert_eq!(r, 1);
            if v == 1 {
                break 'b;
            }

            'd: loop {
                {
                    if v == r {
                        break 'b;
                    }
                    if r == 5 {
                        break 'd;
                    }
                    r += 1;
                }
            }
            assert_eq!(r, 5);
            assert!(v > r);

            return v;
        }
        r
    }
}

trait LabelBreakMatch {
    fn label_break_match(c: u8, xe: u8, ye: i8);
}

impl LabelBreakMatch for () {
    fn label_break_match(c: u8, xe: u8, ye: i8) {
        let mut x = 0;
        let y = 'a: {
            match c {
                0 => break 'a 0,
                v if { if v % 2 == 0 { break 'a 1; }; v % 3 == 0 } => { x += 1; },
                v if { 'b: { break 'b v == 5; } } => { x = 41; },
                _ => 'b: {
                    break 'b ();
                },
            }
            x += 1;
            -1
        };

        assert_eq!(x, xe);
        assert_eq!(y, ye);
    }
}

#[allow(unused_labels)]
trait LabelBreakMacro {
    fn label_break_macro();
}

impl LabelBreakMacro for () {
    fn label_break_macro() {
        macro_rules! mac1 {
            ($target:lifetime, $val:expr) => {
                break $target $val;
            };
        }
        let x: u8 = 'a: {
            'b: {
                mac1!('b, 1);
            };
            0
        };
        assert_eq!(x, 0);
        let x: u8 = 'a: {
            'b: {
                if true {
                    mac1!('a, 1);
                }
            };
            0
        };
        assert_eq!(x, 1);
    }
}

pub fn main() {
    assert_eq!(<() as LabelBreak>::label_break(true, false), 1);
    assert_eq!(<() as LabelBreak>::label_break(false, true), 2);
    assert_eq!(<() as LabelBreak>::label_break(false, false), 3);

    assert_eq!(<() as BreakValue>::break_value(true, false), 1);
    assert_eq!(<() as BreakValue>::break_value(false, true), 2);
    assert_eq!(<() as BreakValue>::break_value(false, false), 3);

    assert_eq!(<() as LabelBreakMixed>::label_break_mixed(0), 0);
    assert_eq!(<() as LabelBreakMixed>::label_break_mixed(1), 1);
    assert_eq!(<() as LabelBreakMixed>::label_break_mixed(2), 2);
    assert_eq!(<() as LabelBreakMixed>::label_break_mixed(3), 3);
    assert_eq!(<() as LabelBreakMixed>::label_break_mixed(4), 4);
    assert_eq!(<() as LabelBreakMixed>::label_break_mixed(5), 5);
    assert_eq!(<() as LabelBreakMixed>::label_break_mixed(6), 6);

    <() as LabelBreakMatch>::label_break_match(0, 0, 0);
    <() as LabelBreakMatch>::label_break_match(1, 1, -1);
    <() as LabelBreakMatch>::label_break_match(2, 0, -1);
    <() as LabelBreakMatch>::label_break_match(3, 1, -1);
    <() as LabelBreakMatch>::label_break_match(4, 1, -1);
    <() as LabelBreakMatch>::label_break_match(5, 0, 5);

    <() as LabelBreakMacro>::label_break_macro();
}