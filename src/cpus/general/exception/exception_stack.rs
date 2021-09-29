use crate::cpus::general::exception::Exception;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExceptionStack(Vec<Exception>);

impl ExceptionStack {
    pub fn push(&mut self, exception: Exception) -> Option<()> {
        if self.0.last().is_none() || exception.get_priority() < self.0.last().unwrap().get_priority() {
            self.0.push(exception);
            return Some(());
        }
        None
    }
}
