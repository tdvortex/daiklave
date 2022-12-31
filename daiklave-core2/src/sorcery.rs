use std::{ops::Deref, env::consts};

use serde::{Serialize, Deserialize};

use crate::{book_reference::BookReference, id::UniqueId, charms::{CharmCost, CharmKeyword}};

pub struct SorceryArchetype {
    name: String, 
    book_reference: Option<BookReference>, 
    description: String,
}

impl SorceryArchetype {
    pub fn new(name: String, book_reference: Option<BookReference>, description: String) -> Self {
        Self {
            name,
            book_reference,
            description,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SorceryArchetypeId(pub UniqueId);

impl Deref for SorceryArchetypeId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ShapingRitual {
    archetype_id: SorceryArchetypeId,
    book_reference: Option<BookReference>,
    description: String,
}

impl ShapingRitual {
    pub fn new(
        archetype_id: SorceryArchetypeId,
        book_reference: Option<BookReference>,
        description: String,
    ) -> Self {
        Self {
            archetype_id,
            book_reference,
            description
        }
    }
}

pub struct ShapingRitualId(pub UniqueId);

impl Deref for ShapingRitualId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Spell {
    name: String,
    book_reference: Option<BookReference>,
    costs: Vec<CharmCost>,
    keywords: Vec<CharmKeyword>,
    duration: String,
    description: String,
}

impl Spell {
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
}

pub struct SpellId(pub UniqueId);

impl Deref for SpellId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct TerrestrialSpell(Spell);

impl TerrestrialSpell {
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