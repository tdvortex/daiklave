use crate::{book_reference::BookReference, merits::merit::MeritType};

use super::with_dots::NonStackableMeritWithDots;

pub(crate) struct NonStackableMeritView<'source>(NonStackableMeritWithDots<'source>);

impl<'source> NonStackableMeritView<'source>{
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
}