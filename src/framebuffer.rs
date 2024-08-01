pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }
}
