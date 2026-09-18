//! VirtIO-GPU / Linear Framebuffer Hardware Abstraction

pub struct FramebufferDriver {
    pub width: u32,
    pub height: u32,
    pub bpp: u32,
    pub base_ptr: *mut u32,
}

impl FramebufferDriver {
    pub fn new(width: u32, height: u32, base_ptr: *mut u32) -> Self {
        Self {
            width,
            height,
            bpp: 32,
            base_ptr,
        }
    }

    pub fn clear(&mut self, color: u32) {
        if !self.base_ptr.is_null() {
            let size = (self.width * self.height) as usize;
            unsafe {
                let slice = core::slice::from_raw_parts_mut(self.base_ptr, size);
                slice.fill(color);
            }
        }
    }
}
