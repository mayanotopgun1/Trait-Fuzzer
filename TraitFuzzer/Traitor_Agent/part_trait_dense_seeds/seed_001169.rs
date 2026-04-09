pub trait SliceMutExt<T> {
    fn as_slice_mut(&mut self) -> &mut [T];
}

impl<T> SliceMutExt<T> for [T] {
    fn as_slice_mut(&mut self) -> &mut [T] {
        self
    }
}

trait SliceMutExtRef: SliceMutExt<u8> + AsMut<[u8]> {}

impl<S> SliceMutExtRef for S where S: SliceMutExt<u8> + AsMut<[u8]> {}

pub fn main() {
    let mut x = [1, 2, 3];
    let _x: &mut [isize] = x.as_slice_mut();
}