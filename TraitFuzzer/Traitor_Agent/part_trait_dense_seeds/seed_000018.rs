#![feature(return_position_impl_trait_in_trait)]

trait UnitTrait { fn unit() -> impl std::fmt::Debug; }
impl UnitTrait for () { fn unit() -> impl std::fmt::Debug { () } }

fn main() {
    let x = ();
    || || || || || || || ||
    || || || || || || || ||
    || || || || || || || ||
    || || || || || || || ||

    || || || || || || || ||
    || || || || || || || ||
    || || || || || || || ||
    || || || || || || || ||

    || || || || || || || ||
    || || || || || || || ||
    || || || || || || || ||
    || || || || || || || ||
    [&(), &x];
}