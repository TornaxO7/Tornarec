pub mod base;
pub mod touchscreen;

pub trait Screen {
    fn new<S: AsRef<str>>(name: S) -> Self;

    fn refresh(&mut self);
}
