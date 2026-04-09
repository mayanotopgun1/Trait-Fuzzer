#![feature(generic_associated_types)]

trait StringExt<'a> {
    type StrType;
    fn is_empty(&self) -> bool;
}

trait EmptyCheckExt<'a>: StringExt<'a> {
    fn is_non_empty(&self) -> bool {
        !self.is_empty()
    }
}

impl<'a, T: StringExt<'a>> EmptyCheckExt<'a> for T {}

impl<'a> StringExt<'a> for String {
    type StrType = &'a str;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

fn main() {
    match Some("hi".to_string()) {
        Some(s) if s.is_non_empty() => {},
        _ => {},
    }
}