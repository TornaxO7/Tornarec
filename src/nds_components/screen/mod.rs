pub mod base;
pub mod touchscreen;

use self::{base::BaseScreen, touchscreen::Touchscreen};

#[derive(Debug)]
pub struct NDSScreens {
    pub top: BaseScreen,
    pub bottom: Touchscreen,
}

pub trait Screen {
    fn new<S: AsRef<str>>(name: S) -> Self;

    fn refresh(&mut self);
}
