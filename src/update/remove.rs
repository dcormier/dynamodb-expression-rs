use core::{fmt, ops};

use crate::name::Name;

// func Remove(name NameBuilder) UpdateBuilder
// func (ub UpdateBuilder) Remove(name NameBuilder) UpdateBuilder

/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.REMOVE>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Remove {
    // TODO: Name or Path?
    names: Vec<Name>,
}

impl<T> FromIterator<T> for Remove
where
    T: Into<Name>,
{
    fn from_iter<I>(names: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            names: names.into_iter().map(Into::into).collect(),
        }
    }
}

impl ops::Deref for Remove {
    type Target = Vec<Name>;

    fn deref(&self) -> &Self::Target {
        &self.names
    }
}

impl ops::DerefMut for Remove {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.names
    }
}

impl fmt::Display for Remove {
    // TODO: Test this
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("REMOVE ")?;

        let mut first = true;
        self.names.iter().try_for_each(|name| {
            if first {
                first = false;
            } else {
                f.write_str(", ")?;
            }

            name.fmt(f)
        })
    }
}
