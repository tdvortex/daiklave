mod id;
pub use id::SpellId;

use serde::{Deserialize, Serialize};

use crate::{
    book_reference::BookReference,
    charms::{CharmCost, CharmKeyword},
};

/// A Sorcery Spell. Note that this is almost never used directly; instead,
/// it is typically wrapped in TerrestrialSpell, CelestialSpell, or SolarSpell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Spell {
    name: String,
    book_reference: Option<BookReference>,
    costs: Vec<CharmCost>,
    keywords: Vec<CharmKeyword>,
    duration: String,
    description: String,
}

impl Spell {
    /// Creates a new Spell
    pub fn new(
        name: String,
        book_reference: Option<BookReference>,
        costs: Vec<CharmCost>,
        keywords: Vec<CharmKeyword>,
        duration: String,
        description: String,
    ) -> Self {
        Self {
            name,
            book_reference,
            costs,
            keywords,
            duration,
            description,
        }
    }

    /// The Spell's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// The book reference for the spell, if any
    pub fn book_reference(&self) -> Option<&BookReference> {
        self.book_reference.as_ref()
    }

    /// The costs required to cast the spell
    pub fn costs(&self) -> &[CharmCost] {
        &self.costs
    }

    /// The keywords of this spell.
    pub fn keywords(&self) -> &[CharmKeyword] {
        &self.keywords
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
