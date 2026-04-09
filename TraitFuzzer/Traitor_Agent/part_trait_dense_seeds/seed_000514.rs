#![allow(bare_trait_objects)]

pub struct FormatWith<'a, I, F> {
    sep: &'a str,

    inner: RefCell<Option<(I, F)>>,
}

use std::cell::RefCell;
use std::fmt;

trait LayoutTrait {
    fn layout(&self) -> String;
}

struct Layout;

impl LayoutTrait for Layout {
    fn layout(&self) -> String {
        "Layout".to_string()
    }
}

pub fn new_format<'a, I, F>(iter: I, separator: &'a str, f: F) -> FormatWith<'a, I, F>
where
    I: Iterator,
    F: FnMut(I::Item, &mut FnMut(&fmt::Display) -> fmt::Result) -> fmt::Result,
{
    FormatWith { sep: separator, inner: RefCell::new(Some((iter, f))) }
}

trait FormatExt<'a, I, F>
where
    I: Iterator,
    F: FnMut(I::Item, &mut FnMut(&fmt::Display) -> fmt::Result) -> fmt::Result,
{
    fn new_format(iter: I, separator: &'a str, f: F) -> Self;
}

impl<'a, I, F> FormatExt<'a, I, F> for FormatWith<'a, I, F>
where
    I: Iterator,
    F: FnMut(I::Item, &mut FnMut(&fmt::Display) -> fmt::Result) -> fmt::Result,
{
    fn new_format(iter: I, separator: &'a str, f: F) -> Self {
        FormatWith { sep: separator, inner: RefCell::new(Some((iter, f))) }
    }
}

fn main() {
    let _ = FormatWith::<_, _>::new_format(0..32, " | ", |i, f| f(&format_args!("0x{:x}", i)));
}