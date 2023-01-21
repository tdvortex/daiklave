/// Builder path for constructing a Spell.
pub mod builder;

mod cost;
mod id;
mod keyword;
use std::collections::HashSet;

pub use id::SpellId;
pub use keyword::SpellKeyword;

use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

use self::cost::SpellCost;

/// A Sorcery Spell. Note that this is almost never used directly; instead,
/// it is typically wrapped in TerrestrialSpell, CelestialSpell, or SolarSpell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Spell {
    name: String,
    summary: Option<String>,
    cost: SpellCost,
    duration: String,
    description: String,
    book_reference: Option<BookReference>,
    keywords: HashSet<SpellKeyword>,
}

impl Spell {
    /// The Spell's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// The book reference for the spell, if any
    pub fn book_reference(&self) -> Option<&BookReference> {
        self.book_reference.as_ref()
    }

    /// The costs required to cast the spell
    pub fn costs(&self) -> &SpellCost {
        &self.cost
    }

    /// The keywords of this spell.
    pub fn keywords(&self) -> impl Iterator<Item = SpellKeyword> + '_ {
        self.keywords.iter().copied()
    }

    /// The duration of the spell effect after casting.
    pub fn duration(&self) -> &str {
        self.duration.as_str()
    }

    /// A description of the spell.
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
}
