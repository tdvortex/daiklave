use crate::{book_reference::BookReference, merits::merit::{MeritType, MeritPrerequisite}};

use super::{with_dots::NonStackableMeritWithDots, NonStackableMerit};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NonStackableMeritView<'source>(pub NonStackableMeritWithDots<'source>);

impl<'source> NonStackableMeritView<'source> {
    pub fn as_memo(&self) -> NonStackableMerit {
        NonStackableMerit(self.0.as_memo())
    }

    pub fn template_name(&self) -> &'source str {
        self.0.template_name()
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.0.book_reference()
    }

    pub fn dots(&self) -> u8 {
        self.0.dots()
    }

    pub fn merit_type(&self) -> MeritType {
        self.0.merit_type()
    }

    pub fn description(&self) -> (&'source str, Option<&'source str>) {
        self.0.description()
    }

    pub fn prerequisites(&self) -> impl ExactSizeIterator<Item = MeritPrerequisite> + '_ {
        self.0.prerequisites()
    }
}
