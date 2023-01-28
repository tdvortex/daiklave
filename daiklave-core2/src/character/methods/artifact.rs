use crate::{
    artifact::{
        wonders::Wonders, AddArtifact, ArtifactName, ArtifactNameMutation, MagicMaterial, Sonance,
    },
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
            AddArtifact::Wonder((name, wonder)) => {
                self.exaltation.add_wonder(name.as_str(), wonder)?;
            }
        }
        Ok(self)
    }

    /// Removes an artifact from the character.
    pub fn remove_artifact(
        &mut self,
        artifact_name: &ArtifactNameMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        match artifact_name {
            ArtifactNameMutation::Weapon(artifact_weapon_name) => {
                self.exaltation
                    .remove_artifact_weapon(artifact_weapon_name.as_str())?;
            }
            ArtifactNameMutation::Armor(artifact_armor_name) => {
                self.exaltation
                    .remove_artifact_armor(artifact_armor_name.as_str())?;
            }
            ArtifactNameMutation::Wonder(wonder_name) => {
                self.exaltation.remove_wonder(wonder_name.as_str())?;
            }
        }
        // May lose evocations along with the artifact
        self.correct_evocations(&[]);
        Ok(self)
    }

    /// Attunes to the specified artifact.
    pub fn attune_artifact(
        &mut self,
        name: ArtifactName<'_>,
        first: MotePoolName,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.attune_artifact(name, first)?;
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
