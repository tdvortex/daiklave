mod add;
mod error;
mod instance;
mod merit_type;
mod prerequisite;
mod remove;
mod source;
mod template;

pub use add::AddMerit;
pub use error::MeritError;
pub use instance::manse;
pub use instance::{
    AddDemense, SorceryArchetypeMeritName,
    AddNonStackableMerit, AddSorceryArchetypeMerit, AddStackableMerit, DemenseName,
    RemoveNonStackableMerit, RemoveSorceryArchetypeMerit, RemoveStackableMerit,
};
pub(crate) use instance::{
    MeritInstanceInner, NonStackableMerit, NonStackableMeritInstance, SorceryArchetypeMerit,
    SorceryArchetypeMeritDetails, StackableMerit, StackableMeritInstance,
};
pub use merit_type::MeritType;
pub use prerequisite::MeritPrerequisite;
pub use remove::RemoveMerit;
pub(crate) use source::MeritSource;
pub use template::{NonStackableMeritTemplateName, NonStackableMeritName, StackableMeritTemplateName};

use crate::book_reference::BookReference;

pub struct Merit<'source>(pub(crate) MeritSource<'source>);

impl<'source> Merit<'source> {
    pub fn name(&self) -> &'source str {
        self.0.name()
    }

    pub fn detail(&self) -> Option<&'source str> {
        self.0.detail()
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.0.book_reference()
    }

    pub fn dots(&self) -> u8 {
        self.0.dots()
    }

    pub fn description(&self) -> &'source str {
        self.0.description()
    }

    pub fn dot_description(&self) -> Option<&'source str> {
        self.0.dot_description()
    }
}
