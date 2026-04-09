trait OptionExt<T> {
    fn extract(&mut self) -> T;
}

impl OptionExt<i32> for Option<i32>
where
    i32: Copy + std::ops::AddAssign + PartialEq,
{
    fn extract(&mut self) -> i32 {
        match self {
            Some(val @ (3 | 4 | 6)) => {
                let result = *val;
                *val += 1;
                result
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    let mut opt = Some(3);
    let mut w = Vec::new();
    while let Some(ref mut val) = opt {
        if [3, 4, 6].contains(val) {
            w.push(opt.extract());
        } else {
            break;
        }
    }
    assert_eq!(w, [3, 4]);
    if let &(None | Some(6 | 7)) = &opt {
        unreachable!();
    }
    if let Some(x @ (4 | 5 | 6)) = opt {
        assert_eq!(x, 5);
    } else {
        unreachable!();
    }
}