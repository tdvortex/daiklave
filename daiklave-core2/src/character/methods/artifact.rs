use crate::{
    artifact::{wonders::Wonders, Artifact, ArtifactId, ArtifactName, MagicMaterial, Sonance},
    exaltation::{exalt::essence::MotePoolName, Exaltation},
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
            Artifact::Weapon(artifact_memo) => {
                self.exaltation
                    .add_artifact_weapon(artifact_memo.0.as_str(), artifact_memo.1.as_ref())?;
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
        artifact_name: &ArtifactName,
    ) -> Result<&mut Self, CharacterMutationError> {
        match artifact_name {
            ArtifactName::Weapon(artifact_weapon_name) => {
                self.exaltation
                    .remove_artifact_weapon(artifact_weapon_name.as_str())?;
            }
            ArtifactName::Armor(artifact_armor_id) => {
                self.exaltation.remove_artifact_armor(*artifact_armor_id)?;
            }
            ArtifactName::Wonder(wonder_id) => {
                self.exaltation.remove_wonder(*wonder_id)?;
            }
        }
        // May lose evocations along with the artifact
        self.correct_evocations(&[]);
        Ok(self)
    }

    /// Attunes to the specified artifact.
    pub fn attune_artifact(
        &mut self,
        artifact_id: ArtifactId<'_>,
        first: MotePoolName,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.attune_artifact(artifact_id, first)?;
        Ok(self)
    }

    /// Checks if the character is Resonant, Dissonant, or neither with a
    /// magic material.
    pub fn sonance(&self, magic_material: MagicMaterial) -> Option<Sonance> {
        if let Exaltation::Exalt(exalt) = &self.exaltation {
            exalt.exalt_type.sonance(magic_material)
        } else {
            None
        }
    }
}
