mod ability;
mod abilities_vanilla;
mod error;
mod memo;
mod set;
mod add_specialty;
mod remove_specialty;

pub(crate) use ability::{AbilityRating, AbilityRatingMemo};
pub(crate) use abilities_vanilla::AbilitiesVanilla;
pub(crate) use error::AbilityError;
pub(crate) use memo::AbilitiesMemo;
pub use ability::{Ability, AbilityName, AbilityNameVanilla, AbilityNameQualified, AbilityNameQualifiedMutation};
pub use add_specialty::AddSpecialty;
pub use remove_specialty::RemoveSpecialty;
pub use set::SetAbility;

use crate::Character;

/// An interface for read-only accessing all character abilities, including
/// Craft and Martial Arts.
pub struct Abilities<'view, 'source>(pub(crate) &'view Character<'source>);

impl<'view, 'source> Abilities<'view, 'source> {
    pub fn get(&'view self, ability_name: AbilityNameQualified<'_>) -> Option<Ability<'view, 'source>> {
        match ability_name {
            AbilityNameQualified::Vanilla(vanilla) => Some(self.get_vanilla(vanilla)),
            AbilityNameQualified::Craft(focus) => self.get_craft(focus),
            AbilityNameQualified::MartialArts(style_name) => self.get_martial_arts(style_name),
        }
    }


    pub(crate) fn get_vanilla(&'view self, vanilla: AbilityNameVanilla) -> Ability<'view, 'source> {
        Ability(AbilityNameQualified::Vanilla(vanilla), self.0.vanilla_abilities().get(vanilla))
    }

    fn get_craft(&'view self, focus: &str) -> Option<Ability<'view, 'source>> {
        self.0.craft().0.get_key_value(focus).map(|(focus, ability_rating)| Ability(AbilityNameQualified::Craft(*focus), ability_rating))
    }

    fn get_martial_arts(&'view self, style_name: &str) -> Option<Ability<'view, 'source>> {
        Some(self.0.martial_arts().style(style_name)?.ability())
    }

    /// Iterates over all abilities, in alphabetical order. Craft and Martial
    /// Arts abilities are further sorted alphabetically by focus area or
    /// style name (respectively).
    pub fn iter(&'view self) -> impl Iterator<Item = Ability<'view, 'source>> + '_ {
        let before_craft = [
            AbilityNameVanilla::Archery,
            AbilityNameVanilla::Athletics,
            AbilityNameVanilla::Awareness,
            AbilityNameVanilla::Brawl,
            AbilityNameVanilla::Bureaucracy,
        ]
        .iter()
        .map(|vanilla| self.get_vanilla(*vanilla));

        let craft = self
            .0
            .craft()
            .iter()
            .filter_map(|focus| self.get_craft(focus));

        let after_craft = [
            AbilityNameVanilla::Dodge,
            AbilityNameVanilla::Integrity,
            AbilityNameVanilla::Investigation,
            AbilityNameVanilla::Larceny,
            AbilityNameVanilla::Linguistics,
            AbilityNameVanilla::Lore,
        ]
        .iter()
        .map(|vanilla| self.get_vanilla(*vanilla));

        let martial_arts = self
            .0
            .martial_arts()
            .iter()
            .filter_map(|style_id| self.get_martial_arts(style_id));

        let after_martial_arts = [
            AbilityNameVanilla::Medicine,
            AbilityNameVanilla::Melee,
            AbilityNameVanilla::Occult,
            AbilityNameVanilla::Performance,
            AbilityNameVanilla::Presence,
            AbilityNameVanilla::Resistance,
            AbilityNameVanilla::Ride,
            AbilityNameVanilla::Sail,
            AbilityNameVanilla::Socialize,
            AbilityNameVanilla::Stealth,
            AbilityNameVanilla::Survival,
            AbilityNameVanilla::Thrown,
            AbilityNameVanilla::War,
        ]
        .iter()
        .map(|vanilla| self.get_vanilla(*vanilla));

        before_craft
            .chain(craft)
            .chain(after_craft)
            .chain(martial_arts)
            .chain(after_martial_arts)
    }
}



