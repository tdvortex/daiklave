use std::ops::Deref;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NonStackableMeritTemplateName(String);

impl<T> From<T> for NonStackableMeritTemplateName where T: Into<String> {
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