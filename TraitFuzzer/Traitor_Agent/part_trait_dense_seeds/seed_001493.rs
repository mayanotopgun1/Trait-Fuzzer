pub trait Associate {
    type Associated;
}

pub struct Wrap<'a> {
    pub field: &'a i32,
}

pub trait Create<T> {
    fn create() -> Self;
}

impl<'a> Associate for Wrap<'a> {
    type Associated = Wrap<'a>;
}

impl<'a, T> Create<T> for Wrap<'a> {
    fn create() -> Self {
        Wrap { field: &0 }
    }
}

pub fn oh_no<'a, T>()
where
    Wrap<'a>: Associate,
    <Wrap<'a> as Associate>::Associated: Create<T>,
{
    <Wrap<'a> as Associate>::Associated::create();
}

pub fn main() {}