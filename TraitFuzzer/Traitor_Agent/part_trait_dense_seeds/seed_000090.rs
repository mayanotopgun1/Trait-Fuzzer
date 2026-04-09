use std::cell::Cell;
use std::ptr::NonNull;

trait ChunkFooterTrait {
    fn prev(&self) -> &Cell<NonNull<ChunkFooter>>;
}

struct ChunkFooter {
    prev: Cell<NonNull<ChunkFooter>>,
}

impl ChunkFooterTrait for ChunkFooter {
    fn prev(&self) -> &Cell<NonNull<ChunkFooter>> {
        &self.prev
    }
}

struct EmptyChunkFooter(ChunkFooter);

unsafe impl Sync for EmptyChunkFooter {}

static EMPTY_CHUNK: EmptyChunkFooter = EmptyChunkFooter(ChunkFooter {
    prev: Cell::new(unsafe {
        NonNull::new_unchecked(&EMPTY_CHUNK as *const EmptyChunkFooter as *mut ChunkFooter)
    }),
});

fn main() {}