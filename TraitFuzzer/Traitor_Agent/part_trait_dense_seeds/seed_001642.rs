#![feature(generic_associated_types, specialization)]

trait StringExt<'a> {
    type StrType;
    fn is_empty(&self) -> bool;
}

default impl<'a, T> StringExt<'a> for T {
    type StrType = &'a str;
    default fn is_empty(&self) -> bool {
        false
    }
}

trait EmptyCheckExt<'a>: StringExt<'a> {
    fn is_non_empty(&self) -> bool {
        !self.is_empty()
    }
}

impl<'a, T: ?Sized + StringExt<'a>> EmptyCheckExt<'a> for T {}

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