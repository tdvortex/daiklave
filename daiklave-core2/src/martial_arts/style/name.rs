use std::ops::Deref;

use crate::martial_arts::charm::builder::MartialArtsCharmBuilder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MartialArtsStyleName(String);

impl MartialArtsStyleName {
    pub fn new_charm(&self) -> MartialArtsCharmBuilder {
        MartialArtsCharmBuilder::style(self.clone())
    }
}

impl<T> From<T> for MartialArtsStyleName where T: Into<String> {
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for MartialArtsStyleName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}