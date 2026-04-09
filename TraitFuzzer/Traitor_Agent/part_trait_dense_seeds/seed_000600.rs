enum Foo {
    VBar { num: isize }
}

struct SBar { num: isize }

trait NumAccess {
    fn get_num(&self) -> isize;
}

impl NumAccess for Foo {
    fn get_num(&self) -> isize {
        match self {
            Foo::VBar { num } => *num,
        }
    }
}

impl NumAccess for SBar {
    fn get_num(&self) -> isize {
        self.num
    }
}

trait NumAdd: NumAccess {
    type Out;
    fn add_num(&self, other: &Self) -> Self::Out;
}

impl NumAdd for Foo {
    type Out = isize;
    fn add_num(&self, other: &Self) -> Self::Out {
        let x = self.get_num();
        let y = other.get_num();
        x + y
    }
}

impl NumAdd for SBar {
    type Out = isize;
    fn add_num(&self, other: &Self) -> Self::Out {
        let x = self.get_num();
        let y = other.get_num();
        x + y
    }
}

pub fn main() {
    let vbar1 = Foo::VBar { num: 1 };
    let vbar2 = Foo::VBar { num: 3 };
    assert_eq!(vbar1.add_num(&vbar2), 4);

    let sbar1 = SBar { num: 2 };
    let sbar2 = SBar { num: 4 };
    assert_eq!(sbar1.add_num(&sbar2), 6);
}