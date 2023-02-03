use std::ops::Deref;

use serde::{Deserialize, Serialize};

use super::SetLimitTrigger;

/// A Limit Trigger for a Celestial Exalt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LimitTrigger(String);

impl LimitTrigger {
    /// Creates a mutation to set a Celestial Exalt's limit trigger.
    pub fn set(self) -> SetLimitTrigger {
        SetLimitTrigger(self)
    }
}

impl<T> From<T> for LimitTrigger
where
    T: Into<String>,
{
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for LimitTrigger {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
