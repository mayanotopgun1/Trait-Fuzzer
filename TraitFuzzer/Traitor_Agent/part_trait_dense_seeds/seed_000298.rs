#![feature(return_position_impl_trait_in_trait)]

#[derive(
    core::clone::Clone,
    core::marker::Copy,
    core::fmt::Debug,
    core::default::Default,
    core::cmp::Eq,
    core::hash::Hash,
    core::cmp::Ord,
    core::cmp::PartialEq,
    core::cmp::PartialOrd,
)]
struct Core;

#[derive(
    std::clone::Clone,
    std::marker::Copy,
    std::fmt::Debug,
    std::default::Default,
    std::cmp::Eq,
    std::hash::Hash,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
)]
struct Std;

trait ColumnTrait {
    fn column(&self) -> Box<dyn core::fmt::Debug>;
}

trait ExtendedColumn: ColumnTrait {
    fn extended_column(&self) -> Box<dyn core::fmt::Debug> {
        self.column()
    }
}

impl<T: ColumnTrait> ExtendedColumn for T {}

impl ColumnTrait for Core {
    fn column(&self) -> Box<dyn core::fmt::Debug> {
        Box::new(*self)
    }
}

impl ColumnTrait for Std {
    fn column(&self) -> Box<dyn core::fmt::Debug> {
        Box::new(*self)
    }
}

fn main() {
    let core: &dyn ExtendedColumn = &Core;
    let std: &dyn ExtendedColumn = &Std;
    core.extended_column();
    std.extended_column();
}