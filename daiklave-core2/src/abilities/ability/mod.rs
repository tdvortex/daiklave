mod name;
mod rating;

pub(crate) use rating::{AbilityRating, AbilityRatingMemo};
pub use name::{AbilityName, AbilityNameQualified, AbilityNameQualifiedMutation, AbilityNameVanilla};

/// An individual ability, whether Craft, Martial Arts, or vanilla.
pub struct Ability<'view, 'source>(pub(crate) AbilityNameQualified<'source>, pub(crate) &'view AbilityRating<'source>);

impl<'view, 'source> Ability<'view, 'source> {
    /// The name of the ability, including the Craft focus or Martial Arts 
    /// style if applicable.
    pub fn name(&self) -> AbilityNameQualified<'source> {
        self.0
    }

    /// The dots rating of the ability.
    pub fn dots(&self) -> u8 {
        self.0.rating().dots()
    }

    /// An iterator over the specialties that the ability has, if any.
    /// Sorted alphabetically.
    pub fn specialties(&self) -> impl Iterator<Item = &'source str> {
        self.0.rating().specialties()
    }
}