//! Bare-Metal Physical Frame Allocator

pub struct FrameAllocator {
    pub start_addr: usize,
    pub total_pages: usize,
    pub next_page: usize,
}

impl FrameAllocator {
    pub fn new(start_addr: usize, total_pages: usize) -> Self {
        Self {
            start_addr,
            total_pages,
            next_page: 0,
        }
    }

    pub fn allocate_frame(&mut self) -> Option<usize> {
        if self.next_page < self.total_pages {
            let addr = self.start_addr + self.next_page * 4096;
            self.next_page += 1;
            Some(addr)
        } else {
            None
        }
    }
}
