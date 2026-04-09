trait PrintExt {
    fn print_with_assignment(&self, value: &mut usize) -> String;
}

impl PrintExt for &str {
    fn print_with_assignment(&self, value: &mut usize) -> String {
        let old_value = *value;
        *value = 5;
        format!("{} is {}", self, old_value)
    }
}

trait FormatExt {
    fn format_with_assignment(&self, value: &mut usize) -> String;
}

impl FormatExt for &str {
    fn format_with_assignment(&self, value: &mut usize) -> String {
        let old_value = *value;
        *value = 5;
        format!("_x is {}", old_value)
    }
}

fn main() {
    let mut _x: usize;
    _x = 1;
    println!("{}", "_x".print_with_assignment(&mut _x));

    let y = _x;
    println!("{}", "y".print_with_assignment(&mut _x));

    println!(
        "first positional arg {}, second positional arg {}, {}",
        1,
        2,
        "y".print_with_assignment(&mut _x)
    );

    _x = 1;
    let mut _f = "_x".format_with_assignment(&mut _x);
    println!("{}", _f);

    let y = _x;
    let mut _f = "y".format_with_assignment(&mut _x);
    println!("{}", _f);

    let mut _f = format!(
        "first positional arg {}, second positional arg {}, {}",
        1, 2, "y".format_with_assignment(&mut _x)
    );
    println!("{}", _f);

    let s = "0.009";
    let width = 5;
    println!(".{:0<width$}", s, width = width);

    let region = "abc";
    let width = 8;
    let ls = "abcde";
    let full = "abcde";

    println!(
        "| {r:rw$?} | {ui:4?} | {v}",
        r = region,
        rw = width,
        ui = ls,
        v = full,
    );

    let a = 4;
    println!("{:.a$}", "aaaaaaaaaaaaaaaaaa", a = a);

    println!("{:._a$}", "aaaaaaaaaaaaaaaaaa", _a = a);
}