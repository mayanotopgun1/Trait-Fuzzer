use std::marker::PhantomData;

pub struct ConcreteError {}

pub trait IoBase {}

struct X {}

impl IoBase for X {}

trait ClusterIteratorTrait<B, E, S = B> {
    fn new(fat: B) -> Self;
}

struct ClusterIterator<B, E, S = B> {
    fat: B,
    phantom_s: PhantomData<S>,
    phantom_e: PhantomData<E>,
}

impl<B: IoBase + 'static, E> ClusterIteratorTrait<B, E> for ClusterIterator<B, E> {
    fn new(fat: B) -> Self {
        ClusterIterator {
            fat,
            phantom_s: PhantomData::default(),
            phantom_e: PhantomData::default(),
        }
    }
}

pub struct FileSystem<IO: IoBase> {
    pub disk: IO,
}

impl<IO: IoBase> FileSystem<IO> {
    pub fn cluster_iter(&self) -> ClusterIterator<impl IoBase + '_, ConcreteError> {
        ClusterIteratorTrait::<X, ConcreteError>::new(X {})
    }
}

fn main() {}