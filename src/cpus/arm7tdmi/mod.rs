use std::time::Duration;

use super::architecture::Architecture;

pub const TICK: Duration = Duration::new(0, 34_000);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Arm7TDMI {
    arch: Architecture,
}
