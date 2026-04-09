#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![allow(dead_code)]

trait Table<const D: usize>: Sync {
    const COLUMNS: usize;
}

struct Table1<const D: usize>;
impl<const D: usize> Table<D> for Table1<D> {
    const COLUMNS: usize = 123;
}

struct Table2<const D: usize>;
impl<const D: usize> Table<D> for Table2<D> {
    const COLUMNS: usize = 456;
}

trait ProcessTableExt<T, const D: usize>
where
    T: Table<D>,
{
    fn process(&self);
}

impl<T, const D: usize> ProcessTableExt<T, D> for T
where
    T: Table<D>,
    [(); T::COLUMNS]:,
{
    fn process(&self) {
        // Placeholder implementation
    }
}

fn process_all_tables<const D: usize>()
where
    [(); Table2::<D>::COLUMNS]:,
    [(); Table1::<D>::COLUMNS]:,
{
    let table1 = Table1::<D>;
    let table2 = Table2::<D>;

    table1.process();
    table2.process();
}

fn main() {}