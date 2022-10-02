use minifb::{Key, KeyRepeat};

use super::{base::BaseScreen, Screen};

#[derive(Debug)]
pub struct Touchscreen {
    screen: BaseScreen,
}

impl Touchscreen {
    pub fn get_pressed_keys(&self) -> Vec<Key> {
        self.screen.window.get_keys_pressed(KeyRepeat::No)
    }
}

impl Screen for Touchscreen {
    fn new<S: AsRef<str>>(name: S) -> Self {
        Self {
            screen: BaseScreen::new(name),
        }
    }

    fn refresh(&mut self) {
        self.screen.refresh();
    }
}
