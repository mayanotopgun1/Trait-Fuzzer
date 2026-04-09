trait CharComparison {
    fn compare_chars(&self, a: char, b: char) -> bool;
}

impl CharComparison for () {
    fn compare_chars(&self, a: char, b: char) -> bool {
        a < b
    }
}

fn main() {
    let result = Box::new(()) as Box<dyn CharComparison>;
    if result.compare_chars('x', 'y') {
        print!("x");
    } else {
        print!("y");
    }
}