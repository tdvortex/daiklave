use std::ops::Deref;

use serde::{Deserialize, Serialize};

/// The name of a non-stackable merit. The name of an individual merit and the
/// template are the same.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NonStackableMeritTemplateName(String);

/// The name of a non-stackable merit. The name of an individual merit and the
/// template are the same.
pub type NonStackableMeritName = NonStackableMeritTemplateName;

impl<T> From<T> for NonStackableMeritTemplateName
where
    T: Into<String>,
{
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for NonStackableMeritTemplateName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
