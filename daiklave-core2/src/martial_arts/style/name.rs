use std::ops::Deref;

use crate::martial_arts::charm::builder::MartialArtsCharmBuilder;

use super::builder::{MartialArtsStyleBuilderWithDescription, MartialArtsStyleBuilder};

/// The name of a Martial Arts style.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MartialArtsStyleName(String);

impl MartialArtsStyleName {
    /// Starts a builder for a new martial arts style with this name and 
    /// description.
    pub fn with_description(self, description: impl Into<String>) -> MartialArtsStyleBuilderWithDescription {
        MartialArtsStyleBuilder::name(self).description(description)
    }

    /// Starts building a new charm for this style.
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