#![feature(box_patterns)]

#[derive(Debug, PartialEq)]
enum MatchArm {
    Arm(usize),
    Wild,
}

#[derive(Debug)]
enum Test {
    Foo,
    Bar,
    Baz,
    Qux,
}

trait Matchable {
    fn match_value(&self) -> MatchArm;
}

impl Matchable for Test {
    fn match_value(&self) -> MatchArm {
        match self {
            Test::Foo | Test::Bar => MatchArm::Arm(0),
            Test::Baz => MatchArm::Arm(1),
            _ => MatchArm::Arm(2),
        }
    }
}

fn test(x: Option<Box<dyn Matchable>>) -> MatchArm {
    match x {
        Some(t) => t.match_value(),
        None => MatchArm::Wild,
    }
}

fn main() {
    assert_eq!(test(Some(Box::new(Test::Foo))), MatchArm::Arm(0));
    assert_eq!(test(Some(Box::new(Test::Bar))), MatchArm::Arm(0));
    assert_eq!(test(Some(Box::new(Test::Baz))), MatchArm::Arm(1));
    assert_eq!(test(Some(Box::new(Test::Qux))), MatchArm::Arm(2));
    assert_eq!(test(None), MatchArm::Wild);
}