/// Builder path for constructing a Spell.
pub mod builder;

mod add;
mod cost;
mod inner;
mod keyword;
mod mutation;
mod name;

use std::{collections::HashSet, num::NonZeroU8};

pub use add::AddSpell;
pub(crate) use inner::SpellInner;
pub use keyword::SpellKeyword;
pub use mutation::{SpellMutation};
pub use name::SpellName;

use crate::book_reference::BookReference;

use self::{builder::SpellBuilder, cost::SpellCost};

use super::{CelestialSpell, SolarSpell, SorceryCircle, TerrestrialSpell};

/// A Spell, grouped by its Circle.
pub enum Spell<'source> {
    /// The First Circle of spells, accessible to all Exalts and some mortals.
    Terrestrial(&'source str, &'source TerrestrialSpell),
    /// The Second Circle of spells, accessible to Solars, Lunars, and
    /// Sidereals.
    Celestial(&'source str, &'source CelestialSpell),
    /// The Third Circle of spells, accessible only to the Chosen of the
    /// Unconquered Sun.
    Solar(&'source str, &'source SolarSpell),
}

impl<'source> Spell<'source> {
    /// Starts constructing a new Spell.
    pub fn with_name(name: impl Into<SpellName>) -> SpellBuilder {
        SpellBuilder {
            name: name.into(),
            book_reference: None,
            summary: None,
            keywords: HashSet::new(),
            control_spell_description: None,
            distortion: None,
        }
    }

    /// The Circle of the spell.
    pub fn circle(&self) -> SorceryCircle {
        match self {
            Spell::Terrestrial(_, _) => SorceryCircle::Terrestrial,
            Spell::Celestial(_, _) => SorceryCircle::Celestial,
            Spell::Solar(_, _) => SorceryCircle::Solar,
        }
    }

    /// The Spell's name.
    pub fn name(&self) -> &'source str {
        match self {
            Spell::Terrestrial(name, _) => *name,
            Spell::Celestial(name, _) => *name,
            Spell::Solar(name, _) => *name,
        }
    }

    /// The book reference for the spell, if any
    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            Spell::Terrestrial(_, terrestrial) => terrestrial.book_reference,
            Spell::Celestial(_, celestial) => celestial.book_reference,
            Spell::Solar(_, solar) => solar.book_reference,
        }
    }

    /// The costs required to cast the spell
    pub fn costs(&self) -> SpellCost {
        match self {
            Spell::Terrestrial(_, terrestrial) => terrestrial.cost,
            Spell::Celestial(_, celestial) => celestial.cost,
            Spell::Solar(_, solar) => solar.cost,
        }
    }

    /// The keywords of this spell.
    pub fn keywords(&self) -> impl Iterator<Item = SpellKeyword> + '_ {
        match self {
            Spell::Terrestrial(_, terrestrial) => terrestrial.keywords.iter().copied(),
            Spell::Celestial(_, celestial) => celestial.keywords.iter().copied(),
            Spell::Solar(_, solar) => solar.keywords.iter().copied(),
        }
    }

    /// The duration of the spell effect after casting.
    pub fn duration(&self) -> &'source str {
        match self {
            Spell::Terrestrial(_, terrestrial) => terrestrial.duration.as_str(),
            Spell::Celestial(_, celestial) => celestial.duration.as_str(),
            Spell::Solar(_, solar) => solar.duration.as_str(),
        }
    }

    /// A description of the spell.
    pub fn description(&self) -> &'source str {
        match self {
            Spell::Terrestrial(_, terrestrial) => terrestrial.description.as_str(),
            Spell::Celestial(_, celestial) => celestial.description.as_str(),
            Spell::Solar(_, solar) => solar.description.as_str(),
        }
    }

    /// Describes the extra effect a sorcerer gets if this is a Control spell.
    pub fn control_spell_description(&self) -> Option<&'source str> {
        match self {
            Spell::Terrestrial(_, terrestrial) => terrestrial.control_spell_description.as_deref(),
            Spell::Celestial(_, celestial) => celestial.control_spell_description.as_deref(),
            Spell::Solar(_, solar) => solar.control_spell_description.as_deref(),
        }
    }

    /// Describes the methods opposing sorcerers may use to distort this spell,
    /// as well as the Goal Number of such attempts.
    pub fn distortion(&self) -> Option<(NonZeroU8, &'source str)> {
        match self {
            Spell::Terrestrial(_, terrestrial) => terrestrial
                .distortion
                .as_ref()
                .map(|(goal_number, text)| (*goal_number, text.as_str())),
            Spell::Celestial(_, celestial) => celestial
                .distortion
                .as_ref()
                .map(|(goal_number, text)| (*goal_number, text.as_str())),
            Spell::Solar(_, solar) => solar
                .distortion
                .as_ref()
                .map(|(goal_number, text)| (*goal_number, text.as_str())),
        }
    }
}
