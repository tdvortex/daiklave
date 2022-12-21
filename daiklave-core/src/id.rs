use std::ops::Deref;

use serde::{Deserialize, Serialize};

/// A unique identifier for a particular resource.
///
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Id {
    Database(i32),
    Placeholder(i32),
}

impl Deref for Id {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Database(i) | Self::Placeholder(i) => i,
        }
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::Placeholder(0)
    }
}

impl Id {
    pub fn is_placeholder(&self) -> bool {
        matches!(self, Self::Placeholder(_))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord, Default)]
pub struct ArmorItemId(pub Id);

impl Deref for ArmorItemId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord, Default)]
pub struct CharacterId(pub Id);

impl Deref for CharacterId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord, Default)]
pub struct AnimaEffectId(pub Id);

impl Deref for AnimaEffectId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord, Default)]
pub struct EvocationId(pub Id);

impl Deref for EvocationId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord, Default)]
pub struct SolarCharmId(pub Id);

impl Deref for SolarCharmId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord, Default)]
pub struct MartialArtsCharmId(pub Id);

impl Deref for MartialArtsCharmId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord, Default)]
pub struct MartialArtsStyleId(pub Id);

impl Deref for MartialArtsStyleId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord, Default)]
pub struct SpellId(pub Id);

impl Deref for SpellId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}