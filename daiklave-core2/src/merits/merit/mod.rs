mod add;
mod error;
mod instance;
mod merit_type;
mod prerequisite;
mod remove;
mod source;
/// Details related to a merit template that may be instantiated into a 
/// character.
pub mod template;

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

use crate::book_reference::BookReference;

/// A merit instance belonging to a character.
pub struct Merit<'source>(pub(crate) MeritSource<'source>);

impl<'source> Merit<'source> {
    /// The name of the merit. This is the base template name,
    /// like "Allies" or "Artifact".
    pub fn name(&self) -> &'source str {
        self.0.name()
    }

    /// If the merit is stackable, the detail of this specific instance.
    pub fn detail(&self) -> Option<&'source str> {
        self.0.detail()
    }

    /// The book reference for the merit, if any.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.0.book_reference()
    }

    /// The dots level for this instance of the merit.
    pub fn dots(&self) -> u8 {
        self.0.dots()
    }

    /// The general description of the benefits of this merit.
    pub fn description(&self) -> &'source str {
        self.0.description()
    }

    /// If the merit is purchasable at multiple dot levels, the description of
    /// this specific dot level.
    pub fn dot_description(&self) -> Option<&'source str> {
        self.0.dot_description()
    }
}
