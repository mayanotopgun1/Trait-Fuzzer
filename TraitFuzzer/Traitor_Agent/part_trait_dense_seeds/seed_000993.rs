#[non_exhaustive]
pub enum NonExhaustiveEnum {
    Unit,
    Tuple(u32),
    Struct { field: u32 }
}

trait EnumMatch {
    fn match_case(&self) -> &'static str;
}

trait EnumDescription: EnumMatch {
    fn describe(&self) -> String {
        format!("The variant is {}", self.match_case())
    }
}

impl<T> EnumDescription for T where T: EnumMatch {}

impl EnumMatch for NonExhaustiveEnum {
    fn match_case(&self) -> &'static str {
        match self {
            NonExhaustiveEnum::Unit => "first",
            NonExhaustiveEnum::Tuple(_) => "second",
            NonExhaustiveEnum::Struct { .. } => "third",
        }
    }
}

fn main() {
    let enum_unit = NonExhaustiveEnum::Unit;
    let _ = enum_unit.describe();
}