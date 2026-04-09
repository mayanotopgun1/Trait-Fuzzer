#![feature(inherent_associated_types, auto_traits, negative_impls)]
#![allow(incomplete_features)]

use std::cmp::Ordering;

trait SelectTrait<T, U> {
    type Type;
}

impl<T: Ordinary, U: Ordinary> SelectTrait<T, U> for Select<T, U> {
    type Type = ();
}

impl<T: Ordinary> SelectTrait<T, Special> for Select<T, Special> {
    type Type = bool;
}

impl<T: Ordinary> SelectTrait<Special, T> for Select<Special, T> {
    type Type = Ordering;
}

impl SelectTrait<Special, Special> for Select<Special, Special> {
    type Type = (bool, bool);
}

struct Select<T, U>(T, U);

fn main() {
    let _: <Select<String, Special> as SelectTrait<String, Special>>::Type = false;
    let _: <Select<Special, Special> as SelectTrait<Special, Special>>::Type = (true, false);
    let _: <Select<Special, u8> as SelectTrait<Special, u8>>::Type = Ordering::Equal;
    let _: <Select<i128, ()> as SelectTrait<i128, ()>>::Type = ();
}

enum Special {}

impl !Ordinary for Special {}

auto trait Ordinary {}