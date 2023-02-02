mod add;
mod remove;
pub use add::AddNonStackableMerit;
pub use remove::RemoveNonStackableMerit;
use serde::{Serialize, Deserialize};

use crate::book_reference::BookReference;

use super::inner::MeritInstanceInner;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NonStackableMeritInstance(pub MeritInstanceInner);

pub struct NonStackableMerit<'source> {
    pub(crate) name: &'source str,
    pub(crate) instance: &'source NonStackableMeritInstance,
}

impl<'source> NonStackableMerit<'source> {
    pub fn name(&self) -> &'source str {
        self.name
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.instance.0.book_reference
    }

    pub fn dots(&self) -> u8 {
        self.instance.0.dots
    }

    pub fn description(&self) -> &'source str {
        &self.instance.0.description
    }

    pub fn dot_description(&self) -> Option<&'source str> {
        self.instance.0.dot_description.as_deref()
    }
}