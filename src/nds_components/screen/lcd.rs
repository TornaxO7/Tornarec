use minifb::{Window, WindowOptions};

use super::Screen;

pub type Pixel = u32;

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 192;
pub const SIZE: usize = WIDTH * HEIGHT;

#[derive(Debug)]
pub struct BaseScreen {
    pub buffer: [Pixel; SIZE],
    window: Window,
}

impl Screen for BaseScreen {
    fn new<S: AsRef<str>>(name: S) -> Self {
        let window = Window::new(name.as_ref(), WIDTH, HEIGHT, WindowOptions::default()).unwrap();

        Self {
            buffer: [Pixel::default(); SIZE],
            window,
        }
    }
    fn refresh(&mut self) {
        self.window
            .update_with_buffer(&self.buffer.as_slice(), WIDTH, HEIGHT)
            .unwrap();
    }
}
