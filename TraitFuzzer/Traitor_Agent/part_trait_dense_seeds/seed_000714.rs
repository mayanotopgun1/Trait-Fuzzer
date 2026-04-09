trait AddLike<Rhs = Self> {
    type Output;
    fn add_like(&self, rhs: Rhs) -> Self::Output;
}

trait SubLike<Rhs = Self> {
    type Output;
    fn sub_like(&self, rhs: Rhs) -> Self::Output;
}

trait MulLike<Rhs = Self> {
    type Output;
    fn mul_like(&self, rhs: Rhs) -> Self::Output;
}

trait DivLike<Rhs = Self> {
    type Output;
    fn div_like(&self, rhs: Rhs) -> Self::Output;
}

trait RemLike<Rhs = Self> {
    type Output;
    fn rem_like(&self, rhs: Rhs) -> Self::Output;
}

trait BitOrLike<Rhs = Self> {
    type Output;
    fn bit_or_like(&self, rhs: Rhs) -> Self::Output;
}

trait BitAndLike<Rhs = Self> {
    type Output;
    fn bit_and_like(&self, rhs: Rhs) -> Self::Output;
}

trait BitXorLike<Rhs = Self> {
    type Output;
    fn bit_xor_like(&self, rhs: Rhs) -> Self::Output;
}

trait ShlLike<Rhs = u32> {
    type Output;
    fn shl_like(&self, rhs: Rhs) -> Self::Output;
}

trait ShrLike<Rhs = u32> {
    type Output;
    fn shr_like(&self, rhs: Rhs) -> Self::Output;
}

impl AddLike<i8> for i8 {
    type Output = i8;
    fn add_like(&self, rhs: i8) -> Self::Output {
        *self + rhs
    }
}

impl SubLike<i16> for i16 {
    type Output = i16;
    fn sub_like(&self, rhs: i16) -> Self::Output {
        *self - rhs
    }
}

impl MulLike<f32> for f32 {
    type Output = f32;
    fn mul_like(&self, rhs: f32) -> Self::Output {
        *self * rhs
    }
}

impl DivLike<f64> for f64 {
    type Output = f64;
    fn div_like(&self, rhs: f64) -> Self::Output {
        *self / rhs
    }
}

impl RemLike<i64> for i64 {
    type Output = i64;
    fn rem_like(&self, rhs: i64) -> Self::Output {
        *self % rhs
    }
}

impl BitOrLike<u8> for u8 {
    type Output = u8;
    fn bit_or_like(&self, rhs: u8) -> Self::Output {
        *self | rhs
    }
}

impl BitAndLike<u16> for u16 {
    type Output = u16;
    fn bit_and_like(&self, rhs: u16) -> Self::Output {
        *self & rhs
    }
}

impl BitXorLike<u32> for u32 {
    type Output = u32;
    fn bit_xor_like(&self, rhs: u32) -> Self::Output {
        *self ^ rhs
    }
}

impl ShlLike<u32> for u64 {
    type Output = u64;
    fn shl_like(&self, rhs: u32) -> Self::Output {
        *self << rhs
    }
}

impl ShrLike<i16> for u64 {
    type Output = u64;
    fn shr_like(&self, rhs: i16) -> Self::Output {
        *self >> rhs as u32
    }
}

fn main() {
    let mut x: i8 = 5;
    x = x.add_like(2);
    assert_eq!(x, 7);

    let mut y: i16 = 10;
    y = y.sub_like(3);
    assert_eq!(y, 7);

    let mut z: f32 = 4.0;
    z = z.mul_like(2.0);
    assert_eq!(z, 8.0);

    let mut w: f64 = 12.0;
    w = w.div_like(3.0);
    assert_eq!(w, 4.0);

    let mut v: i64 = 9;
    v = v.rem_like(4);
    assert_eq!(v, 1);

    let mut u: u8 = 0b1010;
    u = u.bit_or_like(0b1100);
    assert_eq!(u, 0b1110);

    let mut t: u16 = 0b1010;
    t = t.bit_and_like(0b1100);
    assert_eq!(t, 0b1000);

    let mut s: u32 = 0b1010;
    s = s.bit_xor_like(0b1100);
    assert_eq!(s, 0b0110);

    let mut r: u64 = 0b1010;
    r = r.shl_like(2);
    assert_eq!(r, 0b101000);

    let mut q: u64 = 0b101000;
    q = q.shr_like(2);
    assert_eq!(q, 0b1010);
}