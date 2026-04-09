#[derive(Debug, PartialEq)]
enum MatchArm {
    Arm(usize),
    Wild,
}

trait Testable {
    fn test(&self) -> MatchArm;
}

impl Testable for [i32] {
    fn test(&self) -> MatchArm {
        match self {
            [bar @ .., n] if n == &5 => {
                for i in bar {
                    assert!(i < &5);
                }

                MatchArm::Arm(0)
            },
            bar @ [x0, .., xn] => {
                assert_eq!(x0, &1);
                assert_eq!(x0, &1);
                assert_eq!(xn, &4);
                assert_eq!(bar, &[1, 2, 3, 4]);

                MatchArm::Arm(1)
            },
            _ => MatchArm::Wild,
        }
    }
}

fn main() {
    let foo = vec![1, 2, 3, 4, 5];

    assert_eq!(foo.test(), MatchArm::Arm(0));
    assert_eq!((&foo[..4]).test(), MatchArm::Arm(1));
    assert_eq!((&foo[0..1]).test(), MatchArm::Wild);
}