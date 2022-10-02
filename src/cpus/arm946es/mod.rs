use std::time::Duration;
use super::architecture::Architecture;

pub const TICK: Duration = Duration::new(0, 67_000);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Arm949Es {
    arch: Architecture,
}
