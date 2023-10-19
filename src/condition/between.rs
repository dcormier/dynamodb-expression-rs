use core::fmt;

use crate::operand::Operand;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Between {
    pub(crate) op: Operand,

    /// Equal to or greater than this value
    pub(crate) lower: Operand,

    /// Equal to or less than this value
    pub(crate) upper: Operand,
}

impl fmt::Display for Between {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} BETWEEN {} AND {}", self.op, self.lower, self.upper)
    }
}
