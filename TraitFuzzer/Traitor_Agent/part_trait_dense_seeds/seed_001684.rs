trait Mir {
    fn execute(&self);
}

trait Incremental: Mir {
    fn increment(&self) -> usize;
}

impl<T> Incremental for T
where
    T: Mir,
{
    fn increment(&self) -> usize {
        let mut count = 0;
        self.execute();
        while count < 1 {
            count += 1;
        }
        count
    }
}

impl Mir for () {
    fn execute(&self) {
        let x = 1;
        let mut y = 0;
        while y < x {
            y += 1;
        }
    }
}

fn mir() -> impl Mir {
    ()
}

pub fn main() {
    let m = mir();
    let _ = m.increment();
}