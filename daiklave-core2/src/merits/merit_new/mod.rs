mod error;
mod instance;
mod merit_type;
mod prerequisite;
mod source;
mod template;

pub use error::MeritError;
pub use instance::{AddNonStackableMerit, AddSorceryArchetypeMerit, AddStackableMerit, RemoveNonStackableMerit, RemoveSorceryArchetypeMerit, RemoveStackableMerit};
pub use instance::manse;
pub(crate) use instance::{MeritInstanceInner, NonStackableMeritInstance, StackableMeritInstance};
pub use merit_type::MeritType;
pub use prerequisite::MeritPrerequisite;
pub use template::{NonStackableMeritTemplateName, StackableMeritTemplateName};

use crate::book_reference::BookReference;
use source::MeritSource;

pub struct Merit<'source>(MeritSource<'source>);

impl<'source> Merit<'source> {
    pub fn name(&self) -> &'source str {
        todo!()
    }

    pub fn detail(&self) -> Option<&'source str> {
        todo!()
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        todo!()
    }

    pub fn dots(&self) -> u8 {
        todo!()
    }

    pub fn description(&self) -> &'source str {
        todo!()
    }

    pub fn dot_description(&self) -> Option<&'source str> {
        todo!()
    }
}

