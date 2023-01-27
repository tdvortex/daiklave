use crate::{
    artifact::{wonders::Wonders, AddArtifact, ArtifactId, ArtifactName, MagicMaterial, Sonance},
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
        add_artifact: &'source AddArtifact,
    ) -> Result<&mut Self, CharacterMutationError> {
        match add_artifact {
            AddArtifact::Weapon(artifact_weapon) => {
                self.exaltation
                    .add_artifact_weapon(artifact_weapon.0.as_str(), artifact_weapon.1.as_ref())?;
            }
            AddArtifact::Armor((name, artifact_armor)) => {
                self.exaltation
                    .add_artifact_armor(name.as_str(), artifact_armor.as_ref())?;
            }
            AddArtifact::Wonder(wonder_id, wonder) => {
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
            ArtifactName::Armor(artifact_armor_name) => {
                self.exaltation
                    .remove_artifact_armor(artifact_armor_name.as_str())?;
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
