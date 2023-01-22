/// Builder path for constructing a Spell.
pub mod builder;

mod cost;
mod id;
mod inner;
mod keyword;
mod mutation;

use std::{collections::HashSet, num::NonZeroU8};

pub use id::SpellId;
pub(crate) use inner::SpellInner;
pub use keyword::SpellKeyword;
pub use mutation::SpellMutation;

use crate::book_reference::BookReference;

use self::{builder::SpellBuilder, cost::SpellCost};

use super::{CelestialSpell, SolarSpell, SorceryCircle, TerrestrialSpell};

/// A Spell, grouped by its Circle.
pub enum Spell<'source> {
    /// The First Circle of spells, accessible to all Exalts and some mortals.
    Terrestrial(&'source TerrestrialSpell),
    /// The Second Circle of spells, accessible to Solars, Lunars, and
    /// Sidereals.
    Celestial(&'source CelestialSpell),
    /// The Third Circle of spells, accessible only to the Chosen of the
    /// Unconquered Sun.
    Solar(&'source SolarSpell),
}

impl<'source> Spell<'source> {
    /// Starts constructing a new Spell.
    pub fn builder(name: String) -> SpellBuilder {
        SpellBuilder {
            name,
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
            Spell::Terrestrial(_) => SorceryCircle::Terrestrial,
            Spell::Celestial(_) => SorceryCircle::Celestial,
            Spell::Solar(_) => SorceryCircle::Solar,
        }
    }

    /// The Spell's name.
    pub fn name(&self) -> &'source str {
        match self {
            Spell::Terrestrial(terrestrial) => terrestrial.name.as_str(),
            Spell::Celestial(celestial) => celestial.name.as_str(),
            Spell::Solar(solar) => solar.name.as_str(),
        }
    }

    /// The book reference for the spell, if any
    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            Spell::Terrestrial(terrestrial) => terrestrial.book_reference,
            Spell::Celestial(celestial) => celestial.book_reference,
            Spell::Solar(solar) => solar.book_reference,
        }
    }

    /// The costs required to cast the spell
    pub fn costs(&self) -> SpellCost {
        match self {
            Spell::Terrestrial(terrestrial) => terrestrial.cost,
            Spell::Celestial(celestial) => celestial.cost,
            Spell::Solar(solar) => solar.cost,
        }
    }

    /// The keywords of this spell.
    pub fn keywords(&self) -> impl Iterator<Item = SpellKeyword> + '_ {
        match self {
            Spell::Terrestrial(terrestrial) => terrestrial.keywords.iter().copied(),
            Spell::Celestial(celestial) => celestial.keywords.iter().copied(),
            Spell::Solar(solar) => solar.keywords.iter().copied(),
        }
    }

    /// The duration of the spell effect after casting.
    pub fn duration(&self) -> &'source str {
        match self {
            Spell::Terrestrial(terrestrial) => terrestrial.duration.as_str(),
            Spell::Celestial(celestial) => celestial.duration.as_str(),
            Spell::Solar(solar) => solar.duration.as_str(),
        }
    }

    /// A description of the spell.
    pub fn description(&self) -> &'source str {
        match self {
            Spell::Terrestrial(terrestrial) => terrestrial.description.as_str(),
            Spell::Celestial(celestial) => celestial.description.as_str(),
            Spell::Solar(solar) => solar.description.as_str(),
        }
    }

    /// Describes the extra effect a sorcerer gets if this is a Control spell.
    pub fn control_spell_description(&self) -> Option<&'source str> {
        match self {
            Spell::Terrestrial(terrestrial) => terrestrial.control_spell_description.as_deref(),
            Spell::Celestial(celestial) => celestial.control_spell_description.as_deref(),
            Spell::Solar(solar) => solar.control_spell_description.as_deref(),
        }
    }

    /// Describes the methods opposing sorcerers may use to distort this spell,
    /// as well as the Goal Number of such attempts.
    pub fn distortion(&self) -> Option<(NonZeroU8, &'source str)> {
        match self {
            Spell::Terrestrial(terrestrial) => terrestrial
                .distortion
                .as_ref()
                .map(|(goal_number, text)| (*goal_number, text.as_str())),
            Spell::Celestial(celestial) => celestial
                .distortion
                .as_ref()
                .map(|(goal_number, text)| (*goal_number, text.as_str())),
            Spell::Solar(solar) => solar
                .distortion
                .as_ref()
                .map(|(goal_number, text)| (*goal_number, text.as_str())),
        }
    }
}
