#![feature(transmutability)]

trait Transmutable<T> {
    fn assert_transmutable();
}

impl<T> Transmutable<T> for () 
where
    Self: std::mem::TransmuteFrom<T>
{
    fn assert_transmutable() {}
}

enum Uninhabited {}

enum SingleInhabited {
    X,
    Y(Uninhabited)
}

enum SingleUninhabited {
    X(Uninhabited),
    Y(Uninhabited),
}

enum MultipleUninhabited {
    X(u8, Uninhabited),
    Y(Uninhabited, u16),
}

fn main() {
    <() as Transmutable<Uninhabited>>::assert_transmutable();
    <() as Transmutable<SingleInhabited>>::assert_transmutable();
    <() as Transmutable<SingleUninhabited>>::assert_transmutable();
    <() as Transmutable<MultipleUninhabited>>::assert_transmutable();
}