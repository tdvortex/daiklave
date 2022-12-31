use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::{
    book_reference::BookReference,
    charms::{CharmCost, CharmKeyword},
    id::UniqueId,
};

/// A sorcery archetype, representing one path to sorcerous knowledge. This
/// unlocks various shaping rituals as well as unique merits.
pub struct SorceryArchetype {
    name: String,
    book_reference: Option<BookReference>,
    description: String,
}

impl SorceryArchetype {
    /// Creates a new SorceryArchetype.
    pub fn new(name: String, book_reference: Option<BookReference>, description: String) -> Self {
        Self {
            name,
            book_reference,
            description,
        }
    }

    /// The name of the archetype
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// The book reference for the archetype, if any
    pub fn book_reference(&self) -> Option<&BookReference> {
        self.book_reference.as_ref()
    }

    /// A description of the archetype
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
}

/// A unique Id for a Sorcery Archetype
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SorceryArchetypeId(pub UniqueId);

impl Deref for SorceryArchetypeId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A shaping ritual, one method that a sorcerous archetype might use to
/// generate Sorcerous Motes.
pub struct ShapingRitual {
    archetype_id: SorceryArchetypeId,
    book_reference: Option<BookReference>,
    description: String,
}

impl ShapingRitual {
    /// Create a new ShapingRitual
    pub fn new(
        archetype_id: SorceryArchetypeId,
        book_reference: Option<BookReference>,
        description: String,
    ) -> Self {
        Self {
            archetype_id,
            book_reference,
            description,
        }
    }

    /// The Id of the SorceryArchetype associated with this ritual
    pub fn archetype_id(&self) -> SorceryArchetypeId {
        self.archetype_id
    }

    /// The book reference for the shaping ritual, if any
    pub fn book_reference(&self) -> Option<&BookReference> {
        self.book_reference.as_ref()
    }

    /// A description of the shaping ritual
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
}

/// A unique Id for a ShapingRitual
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShapingRitualId(pub UniqueId);

impl Deref for ShapingRitualId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A Sorcery Spell. Note that this is almost never used directly; instead,
/// it is typically wrapped in TerrestrialSpell, CelestialSpell, or SolarSpell.
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

/// A unique Id for a Spell
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpellId(pub UniqueId);

impl Deref for SpellId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A Spell of the first (Terrestrial) Circle. Derefs to Spell.
pub struct TerrestrialSpell(Spell);

impl TerrestrialSpell {
    /// Wraps a Spell as a TerrestrialSpell
    pub fn from_spell(spell: Spell) -> Self {
        Self(spell)
    }
}

impl Deref for TerrestrialSpell {
    type Target = Spell;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
