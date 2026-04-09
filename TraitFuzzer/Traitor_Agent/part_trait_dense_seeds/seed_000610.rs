trait CharComparison {
    fn compare_chars(a: char, b: char) -> bool;
}

impl CharComparison for () {
    fn compare_chars(a: char, b: char) -> bool {
        a < b
    }
}

fn main() {
    let result = <()>::compare_chars('x', 'y');
    if result {
        print!("x");
    } else {
        print!("y");
    }
}