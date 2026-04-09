#![feature(generic_associated_types)]

use std::cell::Cell;
use std::ptr::NonNull;

trait ChunkFooterTrait {
    type Footer<'a> where Self: 'a;

    fn prev<'a>(&'a self) -> &'a Cell<NonNull<Self::Footer<'a>>>;
}

struct ChunkFooter {
    prev: Cell<NonNull<ChunkFooter>>,
}

impl ChunkFooterTrait for ChunkFooter {
    type Footer<'a> = ChunkFooter;

    fn prev<'a>(&'a self) -> &'a Cell<NonNull<Self::Footer<'a>>> {
        &self.prev
    }
}

trait EmptyChunkFooterTrait: ChunkFooterTrait {
    fn empty_chunk() -> &'static Self;
}

impl EmptyChunkFooterTrait for ChunkFooter {
    fn empty_chunk() -> &'static Self {
        unsafe { &EMPTY_CHUNK.0 }
    }
}

struct EmptyChunkFooter(ChunkFooter);

unsafe impl Sync for EmptyChunkFooter {}

static EMPTY_CHUNK: EmptyChunkFooter = EmptyChunkFooter(ChunkFooter {
    prev: Cell::new(unsafe {
        NonNull::new_unchecked(&EMPTY_CHUNK as *const EmptyChunkFooter as *mut ChunkFooter)
    }),
});

fn main() {
    let empty_footer = ChunkFooter::empty_chunk();
    println!("{:p}", empty_footer);
}