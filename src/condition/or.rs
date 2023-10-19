use core::fmt;

use crate::condition::Condition;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Or {
    pub(crate) left: Box<Condition>,
    pub(crate) right: Box<Condition>,
}

impl fmt::Display for Or {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} OR {}", self.left, self.right)
    }
}
