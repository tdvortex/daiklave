use crate::{
    armor::{
        armor_item::{artifact::ArtifactError, ArmorId},
        ArmorError,
    },
    artifact::{wonders::Wonders, Artifact, ArtifactId},
    exaltation::{
        exalt::essence::{EssenceError, MotePoolName},
        Exaltation,
    },
    weapons::{
        weapon::{WeaponId},
        WeaponError,
    },
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    /// Gets the character's artifact Wonders.
    pub fn wonders(&'view self) -> Wonders<'view, 'source> {
        Wonders(&self.exaltation)
    }

    /// Adds an artifact to the character.
    pub fn add_artifact(
        &mut self,
        artifact: &'source Artifact,
    ) -> Result<&mut Self, CharacterMutationError> {
        match artifact {
            Artifact::Weapon(artifact_weapon_id, artifact_memo) => {
                self.exaltation
                    .add_artifact_weapon(*artifact_weapon_id, artifact_memo.0.as_ref())?;
            }
            Artifact::Armor(artifact_armor_id, artifact_memo) => {
                self.exaltation
                    .add_artifact_armor(*artifact_armor_id, artifact_memo.as_ref())?;
            }
            Artifact::Wonder(wonder_id, wonder) => {
                self.exaltation.add_wonder(*wonder_id, wonder)?;
            }
        }
        Ok(self)
    }

    /// Removes an artifact from the character.
    pub fn remove_artifact(
        &mut self,
        artifact_id: ArtifactId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match artifact_id {
            ArtifactId::Weapon(artifact_weapon_id) => {
                self.exaltation.remove_artifact_weapon(artifact_weapon_id)?;
            }
            ArtifactId::Armor(artifact_armor_id) => {
                self.exaltation.remove_artifact_armor(artifact_armor_id)?;
            }
            ArtifactId::Wonder(wonder_id) => {
                self.exaltation.remove_wonder(wonder_id)?;
            }
        }
        Ok(self)
    }

    /// Checks if an artifact can be attuned.
    pub fn check_attune_artifact(
        &self,
        artifact_id: ArtifactId,
        _first: MotePoolName,
    ) -> Result<(), CharacterMutationError> {
        if matches!(self.exaltation, Exaltation::Mortal(_)) {
            return Err(CharacterMutationError::EssenceError(EssenceError::Mortal));
        }

        match artifact_id {
            ArtifactId::Wonder(wonder_id) => {
                let wonder =
                    self.wonders()
                        .get(wonder_id)
                        .ok_or(CharacterMutationError::ArtifactError(
                            ArtifactError::NotFound,
                        ))?;
                if wonder.2.is_some() {
                    Err(CharacterMutationError::EssenceError(
                        EssenceError::AlreadyAttuned,
                    ))
                } else {
                    let cost =
                        wonder
                            .1
                            .attunement_cost
                            .ok_or(CharacterMutationError::EssenceError(
                                EssenceError::NoAttunementCost,
                            ))?;
                    if self.essence().map_or(0, |essence| {
                        essence.motes().peripheral().available()
                            + essence.motes().personal().available()
                    }) < cost
                    {
                        Err(CharacterMutationError::EssenceError(
                            EssenceError::InsufficientMotes,
                        ))
                    } else {
                        Ok(())
                    }
                }
            }
            ArtifactId::Armor(artifact_armor_id) => {
                let armor_item = self
                    .armor()
                    .get(ArmorId::Artifact(artifact_armor_id))
                    .ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?;
                if armor_item.is_attuned() {
                    Err(CharacterMutationError::EssenceError(
                        EssenceError::AlreadyAttuned,
                    ))
                } else if let Some(cost) = armor_item.attunement_cost() {
                    if self.essence().map_or(0, |essence| {
                        essence.motes().peripheral().available()
                            + essence.motes().personal().available()
                    }) < cost
                    {
                        Err(CharacterMutationError::EssenceError(
                            EssenceError::InsufficientMotes,
                        ))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(CharacterMutationError::EssenceError(
                        EssenceError::NoAttunementCost,
                    ))
                }
            }
            ArtifactId::Weapon(artifact_weapon_id) => {
                let weapon = self
                    .weapons()
                    .iter()
                    .find_map(|(weapon_id, equipped)| {
                        if weapon_id == WeaponId::Artifact(artifact_weapon_id) {
                            self.weapons().get(weapon_id, equipped)
                        } else {
                            None
                        }
                    })
                    .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?;

                if weapon.is_attuned() {
                    Err(CharacterMutationError::EssenceError(
                        EssenceError::AlreadyAttuned,
                    ))
                } else if weapon.is_artifact() {
                    if self.essence().map_or(0, |essence| {
                        essence.motes().peripheral().available()
                            + essence.motes().personal().available()
                    }) < 5
                    {
                        Err(CharacterMutationError::EssenceError(
                            EssenceError::InsufficientMotes,
                        ))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(CharacterMutationError::WeaponError(WeaponError::NotFound))
                }
            }
        }
    }

    /// Attunes to the specified artifact.
    pub fn attune_artifact(
        &mut self,
        artifact_id: ArtifactId,
        first: MotePoolName,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.attune_artifact(artifact_id, first)?;
        Ok(self)
    }
}
