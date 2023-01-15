pub(crate) mod constants;

mod id;
pub use id::MeritId;

mod merit_type;
pub use merit_type::MeritType;

mod nonstackable;
pub use nonstackable::NonStackableMerit;

mod prerequisite;

mod source;

mod stackable;
pub use stackable::{StackableMerit, StackableMeritId};

mod template;

use crate::{book_reference::BookReference};

use self::{template::MeritTemplateId, source::MeritSource};

/// A single Merit possessed by a character.
pub struct Merit<'source>(MeritSource<'source>);

impl<'source> Merit<'source> {
    /// The Id of this instance of the merit as owned by the character.
    pub fn id(&self) -> MeritId {
        self.0.id()
    }

    /// The Id of the merit template which this merit instantiates.
    pub fn template_id(&self) -> MeritTemplateId {
        self.0.template_id()
    }

    /// The name of the merit, excluding any detailing. For example, this would
    /// be "Allies" not "Allies (Ragara Kvin)".
    pub fn template_name(&self) -> &'source str {
        self.0.template_name()
    }

    /// The book reference for the merit, if any.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.0.book_reference()
    }

    /// If the merit is stackable, the detail element describing this unique
    /// instance. For example, if the instance were "Allies (Ragara Kvin)", 
    /// this would return Some("Ragara Kvin").
    pub fn detail(&self) -> Option<&'source str> {
        self.0.detail()
    }

    /// The number of dots for this merit. If the merit is an N/A artifact,
    /// will return 6; otherwise will be between 0 and 5 (inclusive).
    pub fn dots(&self) -> u8 {
        self.0.dots()
    }

    /// Whether the merit is Innate, Purchased, Story, or Supernatural.
    pub fn merit_type(&self) -> MeritType {
        self.0.merit_type()
    }

    /// Returns the description of the merit in two parts. The first element is
    /// the general merit description. If the merit is purchasable at multiple
    /// dot levels, then the second element will be Some with a description of
    /// the effect at this specific dot level. If the merit only has one level,
    /// will return (&str, None).
    pub fn description(&self) -> (&'source str, Option<&'source str>) {
        self.0.description()
    }
}










