use crate::{
    armor::{
        armor_item::{mundane::{AddMundaneArmor}, ArmorName, artifact::AddArtifactArmor, AddArmor},
        Armor,
    },
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    /// The character's Armor items.
    pub fn armor(&'view self) -> Armor<'view, 'source> {
        Armor(&self.exaltation)
    }

    /// Add a piece of artifact armor to a character.
    pub fn add_artifact_armor(
        &mut self,
        add_artifact_armor: &'source AddArtifactArmor
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_artifact_armor(add_artifact_armor.name.as_str(), (&add_artifact_armor.armor).into())?;
        Ok(self)
    }

    /// Adds a piece of mundane armor to a character.
    pub fn add_mundane_armor(
        &mut self,
        add_mundane_armor: &'source AddMundaneArmor,
    ) -> Result<&mut Self, CharacterMutationError> {
        let AddMundaneArmor {
            name,
            armor,
        } = add_mundane_armor;
        self.exaltation.add_mundane_armor(name, armor)?;
        Ok(self)
    }

    /// Add a piece of armor to a character.
    pub fn add_armor(
        &mut self,
        add_armor: &'source AddArmor
    ) -> Result<&mut Self, CharacterMutationError> {
        match &add_armor {
            AddArmor::Artifact(add_artifact_armor) => self.add_artifact_armor(add_artifact_armor),
            AddArmor::Mundane(add_mundane_armor) => self.add_mundane_armor(add_mundane_armor),
        }
    }

    /// Removes a piece of mundane armor from a character.
    pub fn remove_mundane_armor(
        &mut self,
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.remove_mundane_armor(name)?;
        Ok(self)
    }

    /// Equips a specific piece of armor to a character.
    pub fn equip_armor(
        &mut self,
        armor_name: ArmorName<'_>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.equip_armor(armor_name)?;
        Ok(self)
    }

    /// Unequips the currently-equipped piece of armor.
    pub fn unequip_armor(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.unequip_armor()?;
        Ok(self)
    }
}
