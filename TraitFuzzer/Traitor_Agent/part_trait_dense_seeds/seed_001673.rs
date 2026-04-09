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
    let mut _x = 0;

    let f1 = "_x".print_with_assignment(&mut _x);
    println!("{}", f1);

    let f2 = "y".print_with_assignment(&mut _x);
    println!("{}", f2);

    let f3 = format!("{} is {}", "y", _x);
    println!("{}", f3);

    let s = "0.009";
    let width = 5;
    println!(".{:0<width$}", s, width = width);

    let region = "abc";
    let width = 8;
    let ls = "abcde";
    let full = "abcde";

    println!("| {r:rw$?} | {ui:4?} | {v}", r = region, rw = width, ui = ls, v = full);

    let a = 4;
    println!("{:.a$}", "aaaaaaaaaaaaaaaaaa", a = a);

    println!("{:._a$}", "aaaaaaaaaaaaaaaaaa", _a = a);
}