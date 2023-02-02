use crate::book_reference::BookReference;

use super::inner::MeritInstanceInner;

mod add;
mod remove;

pub use add::AddStackableMerit;
pub use remove::RemoveStackableMerit;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct StackableMeritInstance(pub MeritInstanceInner);

pub struct StackableMerit<'source> {
    pub(crate) template_name: &'source str,
    pub(crate) detail: &'source str,
    pub(crate) instance: &'source StackableMeritInstance,
}

impl<'source> StackableMerit<'source> {
    pub fn name(&self) -> &'source str {
        self.template_name
    }

    pub fn detail(&self) -> Option<&'source str> {
        Some(self.detail)
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