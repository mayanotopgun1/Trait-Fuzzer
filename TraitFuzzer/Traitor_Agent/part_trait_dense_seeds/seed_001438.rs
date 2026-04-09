trait MacroTrait {}
struct OutsideStruct;
impl MacroTrait for OutsideStruct {}

fn run(x: &dyn MacroTrait) -> bool {
    true
}

fn main() {
    let outside = OutsideStruct;
    let _ = run(&outside);
}