use crate::{charms::charm::evocation::{Evocation, EvocationId, EvokableId}, Character, CharacterMutationError, artifact::ArtifactId, armor::{armor_item::{ArmorId, artifact::ArtifactError}, ArmorError}, weapons::{weapon::WeaponId, WeaponError}, hearthstones::HearthstoneError};

impl<'source> Character<'source> {
    /// Adds an evocation to the character.
    pub fn add_evocation(&mut self, evocation_id: EvocationId, evocation: &'source Evocation) -> Result<&mut Self, CharacterMutationError> {
        match evocation.evokable_id() {
            EvokableId::Hearthstone(hearthstone_id) => {
                if self.hearthstones().get(hearthstone_id).is_none() {
                    return Err(CharacterMutationError::HearthstoneError(HearthstoneError::NotFound));
                }
            },
            EvokableId::Artifact(ArtifactId::Armor(artifact_armor_id)) => {
                if self.armor().get(ArmorId::Artifact(artifact_armor_id)).is_none() {
                    return Err(CharacterMutationError::ArmorError(ArmorError::NotFound));
                }
            },
            EvokableId::Artifact(ArtifactId::Weapon(artifact_weapon_id)) => {
                if !self.weapons().iter().any(|(weapon_id, _)| weapon_id == WeaponId::Artifact(artifact_weapon_id)) {
                    return Err(CharacterMutationError::WeaponError(WeaponError::NotFound));
                }
            }
            EvokableId::Artifact(ArtifactId::Wonder(wonder_id)) => {
                if self.wonders().get(wonder_id).is_none() {
                    return Err(CharacterMutationError::ArtifactError(ArtifactError::NotFound));
                }
            },
        };

        self.exaltation.add_evocation(evocation_id, evocation)?;
        Ok(self)
    }

    /// Removes an evocation from the character.
    pub fn remove_evocation(&mut self, evocation_id: EvocationId) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }
}