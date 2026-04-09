#![feature(generic_associated_types)]

trait WidthInfo<'a> {
    type Width;
    fn width(&self) -> Option<Self::Width>;
}

trait HasSize {
    fn get_size(&mut self, n: usize) -> usize;
}

trait OtherInfo<'a>: WidthInfo<'a, Width = usize> + HasSize {
    type Info;
    fn get_other(&mut self) -> Self::Info;
}

impl<'a> WidthInfo<'a> for HasInfo {
    type Width = usize;
    fn width(&self) -> Option<Self::Width> {
        self.width
    }
}

impl HasSize for HasInfo {
    fn get_size(&mut self, n: usize) -> usize {
        n
    }
}

impl<'a> OtherInfo<'a> for HasInfo {
    type Info = usize;
    fn get_other(&mut self) -> Self::Info {
        let width = self.width().expect("Width is not available");
        self.get_size(width)
    }
}

struct HasInfo {
    width: Option<usize>,
}

fn main() {
    println!("hello?");
}