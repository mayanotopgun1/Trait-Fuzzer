trait SliceExt<'a> {
    fn slice_from_one(&self) -> &'a str;
}

impl<'a> SliceExt<'a> for &'a str {
    fn slice_from_one(&self) -> &'a str {
        &self[1..]
    }
}

fn main() {
    let s = "";
    let _ = (&s as &dyn SliceExt).slice_from_one();
}