mod memo;
mod abilities_vanilla;
mod ability_name;
mod ability_name_vanilla;
mod ability_rating;
mod ability_rating_memo;
mod error;

pub(crate) use memo::AbilitiesMemo;
pub(crate) use abilities_vanilla::AbilitiesVanilla;
pub use ability_name::AbilityName;
pub use ability_name_vanilla::AbilityNameVanilla;
pub(crate) use ability_rating::AbilityRating;
pub(crate) use ability_rating_memo::AbilityRatingMemo;
pub(crate) use error::{AbilityError};

use crate::{martial_arts::MartialArtsStyleId, Character};

/// An interface for read-only accessing all character abilities, including
/// Craft and Martial Arts.
pub struct Abilities<'view, 'source>(pub(crate) &'view Character<'source>);

impl<'view, 'source> Abilities<'view, 'source> {
    /// Get a "vanilla" ability (i.e. not Craft or Martial Arts).
    pub fn get(&'view self, ability_name: AbilityNameVanilla) -> Ability<'view, 'source> {
        Ability(AbilityType::Vanilla(
            ability_name,
            self.0.vanilla_abilities().get(ability_name),
        ))
    }

    /// Get a craft ability, by specifying the specific artisanal focus.
    /// Returns None (not zero) if there are no dots in that ability.
    pub fn craft(&'view self, focus: &'source str) -> Option<Ability<'view, 'source>> {
        Some(Ability(AbilityType::Craft(
            focus,
            self.0.craft().0.get(focus)?,
        )))
    }

    /// Get a Martial Arts ability. Returns None if the character does not have
    /// the Martial Artist merit for this style; returns 0 if the character has
    /// the Martial Artis merit, but does not have any dots.
    pub fn martial_arts(
        &'view self,
        style_id: MartialArtsStyleId,
    ) -> Option<Ability<'view, 'source>> {
        let rating_ptr = match &self.0.exaltation {
            crate::exaltation::Exaltation::Mortal(mortal) => {
                mortal.martial_arts_styles.get(&style_id)?.ability()
            }
            crate::exaltation::Exaltation::Exalt(exalt) => {
                exalt.martial_arts_styles().get(&style_id)?.ability()
            }
        };
        Some(Ability(AbilityType::MartialArts(style_id, rating_ptr)))
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
        .map(|ability_name| self.get(*ability_name));

        let craft = self
            .0
            .craft()
            .iter()
            .map(|focus| self.craft(focus).unwrap());

        let after_craft = [
            AbilityNameVanilla::Dodge,
            AbilityNameVanilla::Integrity,
            AbilityNameVanilla::Investigation,
            AbilityNameVanilla::Larceny,
            AbilityNameVanilla::Linguistics,
            AbilityNameVanilla::Lore,
        ]
        .iter()
        .map(|ability_name| self.get(*ability_name));

        let martial_arts = self
            .0
            .martial_arts()
            .iter()
            .map(|style_id| self.martial_arts(style_id).unwrap());

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
        .map(|ability_name| self.get(*ability_name));

        before_craft
            .chain(craft)
            .chain(after_craft)
            .chain(martial_arts)
            .chain(after_martial_arts)
    }
}

/// An individual ability, whether Craft, Martial Arts, or vanilla.
pub struct Ability<'view, 'source>(pub(crate) AbilityType<'view, 'source>);

impl<'view, 'source> Ability<'view, 'source> {
    /// The name of the ability.
    pub fn name(&self) -> AbilityName {
        self.0.name()
    }

    /// If the ability is a Craft ability, the focus area of the ability.
    /// None for vanilla and Martial Arts.
    pub fn craft_focus(&self) -> Option<&'source str> {
        self.0.craft_focus()
    }

    /// If the ability is a Martial Arts ability, the Id of the Martial Arts
    /// style (which can be used to look up the name or other parameters).
    /// None for vanilla and Craft.
    pub fn martial_arts_style(&self) -> Option<MartialArtsStyleId> {
        self.0.martial_arts_style()
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

pub(crate) enum AbilityType<'view, 'source> {
    Vanilla(AbilityNameVanilla, &'view AbilityRating<'source>),
    Craft(&'source str, &'view AbilityRating<'source>),
    MartialArts(MartialArtsStyleId, &'view AbilityRating<'source>),
}

impl<'view, 'source> AbilityType<'view, 'source> {
    fn name(&self) -> AbilityName {
        match self {
            AbilityType::Vanilla(name, _) => (*name).into(),
            AbilityType::Craft(_, _) => AbilityName::Craft,
            AbilityType::MartialArts(_, _) => AbilityName::MartialArts,
        }
    }

    fn craft_focus(&self) -> Option<&'source str> {
        match self {
            AbilityType::Craft(focus, _) => Some(*focus),
            _ => None,
        }
    }

    fn martial_arts_style(&self) -> Option<MartialArtsStyleId> {
        match self {
            AbilityType::MartialArts(id, _) => Some(*id),
            _ => None,
        }
    }

    fn rating(&self) -> &'view AbilityRating<'source> {
        match self {
            AbilityType::Vanilla(_, rating)
            | AbilityType::Craft(_, rating)
            | AbilityType::MartialArts(_, rating) => rating,
        }
    }
}
