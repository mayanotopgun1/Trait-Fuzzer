use std::sync::atomic::{AtomicUsize, Ordering};
use std::mem;

struct SetOnDrop<'a>(&'a AtomicUsize, #[allow(dead_code)] [u8; 64]);
impl<'a> Drop for SetOnDrop<'a> {
    fn drop(&mut self) {
        self.0.store(self.0.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
    }
}

trait TypeEq<V: ?Sized> {}
impl<T: ?Sized> TypeEq<T> for T {}

fn assert_types_eq<U: ?Sized, V: ?Sized>() where U: TypeEq<V> {}

trait SyncTrait {
    fn assert_sync(&self) {}
}
impl<T: Sync> SyncTrait for T {}

trait SendSyncTrait {
    fn assert_send_sync(&self) {}
}
impl<T: Send + Sync> SendSyncTrait for T {}

fn main() {

    assert_types_eq::<dyn Sync, dyn Sync + Sync>();
    assert_types_eq::<dyn Sync + Send, dyn Send + Sync>();
    assert_types_eq::<dyn Sync + Send + Sync, dyn Send + Sync>();



    let c = AtomicUsize::new(0);
    {
        let d: Box<dyn Sync> = Box::new(SetOnDrop(&c, [0; 64]));

        assert_eq!(mem::size_of_val(&*d),
                   mem::size_of::<SetOnDrop>());
        assert_eq!(mem::align_of_val(&*d),
                   mem::align_of::<SetOnDrop>());
        assert_eq!(c.load(Ordering::Relaxed), 0);
    }
    assert_eq!(c.load(Ordering::Relaxed), 1);
}