struct Point(isize, isize);

trait Coordinate {
    fn get_x(&self) -> isize;
    fn set_x(&mut self, x: isize);
    fn get_y(&self) -> isize;
    fn set_y(&mut self, y: isize);
}

impl Coordinate for Point {
    fn get_x(&self) -> isize {
        self.0
    }
    fn set_x(&mut self, x: isize) {
        self.0 = x;
    }
    fn get_y(&self) -> isize {
        self.1
    }
    fn set_y(&mut self, y: isize) {
        self.1 = y;
    }
}

fn main() {
    let mut x = Point(3, 2);
    assert_eq!(x.get_x(), 3);
    assert_eq!(x.get_y(), 2);
    x.set_x(x.get_x() + 5);
    assert_eq!(x.get_x(), 8);
    {
        let ry = &mut x;
        ry.set_y(ry.get_y() - 2);
        x.set_x(x.get_x() + 3);
        assert_eq!(x.get_x(), 11);
    }
    assert_eq!(x.get_y(), 0);

    let mut x: (isize, isize) = (3, 2);
    assert_eq!(x.0, 3);
    assert_eq!(x.1, 2);
    x.0 += 5;
    assert_eq!(x.0, 8);
    {
        let ry = &mut x;
        *ry = (ry.0, ry.1 - 2);
        x.0 += 3;
        assert_eq!(x.0, 11);
    }
    assert_eq!(x.1, 0);

}