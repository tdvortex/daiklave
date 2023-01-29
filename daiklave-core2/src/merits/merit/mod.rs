mod add;
pub use add::AddMerit;

mod manse_name;
pub use manse_name::ManseName;

/// Builder path for making new merits.
pub mod builder;

pub(crate) mod constants;

mod demense_name;
pub use demense_name::DemenseName;

mod error;
pub use error::MeritError;

mod name;
pub use name::MeritInstanceName;

mod merit_type;
pub use merit_type::MeritType;

mod nonstackable;
pub(crate) use nonstackable::NonStackableMeritView;
pub use nonstackable::{NonStackableMerit};

mod prerequisite;
pub use prerequisite::MeritPrerequisite;

mod source;
pub(crate) use source::MeritSource;

mod stackable;
pub(crate) use stackable::StackableMeritView;
pub use stackable::{
    StackableMerit, StackableMeritTemplate,
};

mod template;
pub use template::MeritTemplateId;

use crate::book_reference::BookReference;

use self::builder::MeritTemplateBuilder;

/// A single Merit possessed by a character.
pub struct Merit<'source>(pub(crate) MeritSource<'source>);

impl<'source> Merit<'source> {
    /// Starts building a new merit template, which may be stackable,
    /// nonstackable, or a sorcery archetype merit.
    pub fn new_template(name: String) -> MeritTemplateBuilder {
        MeritTemplateBuilder {
            name,
            book_reference: None,
            prerequisites: Vec::new(),
        }
    }

    /// The Id of this instance of the merit as owned by the character.
    pub fn id(&self) -> MeritInstanceName {
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
