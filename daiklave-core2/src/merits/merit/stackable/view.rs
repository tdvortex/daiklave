use crate::{merits::merit::{template::MeritTemplateId, MeritType}, book_reference::BookReference};

use super::{with_dots::StackableMeritWithDots, StackableMerit};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StackableMeritView<'source> {
    pub detail: &'source str,
    pub dotted: StackableMeritWithDots<'source>,
}

impl<'source> StackableMeritView<'source> {
    pub fn as_memo(&self) -> StackableMerit {
        StackableMerit { detail: self.detail.to_owned(), dotted: self.dotted.as_memo() }
    }

    pub fn template_id(&self) -> MeritTemplateId {
        self.dotted.template_id()
    }

    pub fn template_name(&self) -> &'source str {
        self.dotted.template_name()
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.dotted.book_reference()
    }

    pub fn detail(&self) -> &'source str {
        self.detail
    }

    pub fn dots(&self) -> u8 {
        self.dotted.dots()
    }

    pub fn merit_type(&self) -> MeritType {
        self.dotted.merit_type()
    }

    pub fn description(&self) -> (&'source str, Option<&'source str>) {
        self.dotted.description()
    }
}