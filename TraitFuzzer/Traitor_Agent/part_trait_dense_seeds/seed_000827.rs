pub fn main() {
    let _x: fn() = handle_debug_column;
}

fn handle_debug_column() {
    let sampler: Box<dyn Sampler> = Box::new(sample_columns());

    let foo = || {
        sampler.get(17);
    };
    foo();
}

fn sample_columns() -> ColumnGen {
    ColumnGen {}
}

struct ColumnGen {}

trait Sampler {
    fn get(&self, index: i32);
}

impl Sampler for ColumnGen {
    fn get(&self, _index: i32) {}
}