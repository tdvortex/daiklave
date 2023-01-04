use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::unique_id::UniqueId;

/// A unique identifier for either a mortal weapon (e.g. sword) or a base
/// artifact weapon (e.g. daiklave)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct BaseWeaponId(pub UniqueId);

impl Deref for BaseWeaponId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A magic, Essence-empowered weapon.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct ArtifactWeaponId(pub UniqueId);

impl Deref for ArtifactWeaponId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The Id for a weapon.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum WeaponId {
    /// All characters have the Unarmed weapon for free, and it cannot
    /// be removed.
    Unarmed,
    /// A mundane weapon without artifact traits.
    Mundane(BaseWeaponId),
    /// A unique magical weapon.
    Artifact(ArtifactWeaponId),
}

/// The Id for a magical creation (weapon, armor, warstrider, or wonder).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum ArtifactId {
    /// An artifact weapon's Id.
    Weapon(ArtifactWeaponId),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct HearthstoneId(pub UniqueId);

impl Deref for HearthstoneId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The Id for an item which is capable of having Evocations.
pub enum _EvokableItemId {
    /// Hearthstones may have unlockable Evocations.
    Hearthstone(HearthstoneId),
    /// Artifacts may have unlockable Evocations.
    Artifact(ArtifactId),
}
