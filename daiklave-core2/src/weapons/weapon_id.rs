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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct ArtifactWeaponId(pub UniqueId);

impl Deref for ArtifactWeaponId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum WeaponId {
    Unarmed,
    Mundane(BaseWeaponId),
    Artifact(ArtifactWeaponId),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum ArtifactId {
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

pub enum EvokableItemId {
    Hearthstone(HearthstoneId),
    Artifact(ArtifactId),
}