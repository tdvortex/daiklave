use crate::{Character, armor::{Armor, armor_item::{BaseArmorId, mundane::MundaneArmor, ArmorId}, ArmorError}, CharacterMutationError};

impl<'view, 'source> Character<'source> {
    /// The character's Armor items.
    pub fn armor(&'view self) -> Armor<'view, 'source> {
        Armor(&self.exaltation)
    }

    /// Adds a piece of mundane armor to a character.
    pub fn add_mundane_armor(
        &mut self,
        armor_id: BaseArmorId,
        armor: &'source MundaneArmor,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_mundane_armor(armor_id, armor)?;
        self.exaltation.add_mundane_armor(armor_id, armor)?;
        Ok(self)
    }

    /// Removes a piece of mundane armor from a character.
    pub fn remove_mundane_armor(
        &mut self,
        armor_id: BaseArmorId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_remove_mundane_armor(armor_id)?;
        self.exaltation.remove_mundane_armor(armor_id)?;
        Ok(self)
    }

    /// Checks if a piece of armor can be added to a character. The armor item
    /// must be unique (e.g. can't have 2 breastplates)
    pub fn check_add_mundane_armor(
        &self,
        armor_id: BaseArmorId,
        _armor: &'source MundaneArmor,
    ) -> Result<(), CharacterMutationError> {
        if self.armor().get(ArmorId::Mundane(armor_id)).is_some() {
            Err(CharacterMutationError::ArmorError(
                ArmorError::DuplicateArmor,
            ))
        } else {
            Ok(())
        }
    }

    /// Checks if a piece of mundane armor can be removed from a character. The
    /// item must exist, and must be unequipped.
    pub fn check_remove_mundane_armor(
        &self,
        armor_id: BaseArmorId,
    ) -> Result<(), CharacterMutationError> {
        if let Some(armor) = self.armor().get(ArmorId::Mundane(armor_id)) {
            if armor.is_equipped() {
                Err(CharacterMutationError::ArmorError(
                    ArmorError::RemoveEquipped,
                ))
            } else {
                Ok(())
            }
        } else {
            Err(CharacterMutationError::ArmorError(ArmorError::NotFound))
        }
    }

    /// Checks if a piece of armor can be equipped. The armor must exist, and
    /// must not already be equipped.
    pub fn check_equip_armor(&self, armor_id: ArmorId) -> Result<(), CharacterMutationError> {
        if let Some(armor) = self.armor().get(armor_id) {
            if armor.is_equipped() {
                Err(CharacterMutationError::ArmorError(
                    ArmorError::AlreadyEquipped,
                ))
            } else {
                Ok(())
            }
        } else {
            Err(CharacterMutationError::ArmorError(ArmorError::NotFound))
        }
    }

    /// Equips a specific piece of armor to a character.
    pub fn equip_armor(&mut self, armor_id: ArmorId) -> Result<&mut Self, CharacterMutationError> {
        self.check_equip_armor(armor_id)?;
        self.exaltation.equip_armor(armor_id)?;
        Ok(self)
    }

    /// Checks if there is any armor to unequip.
    pub fn check_unequip_armor(&self) -> Result<(), CharacterMutationError> {
        if self.armor().worn().is_none() {
            Err(CharacterMutationError::ArmorError(ArmorError::NotFound))
        } else {
            Ok(())
        }
    }

    /// Unequips the currently-equipped piece of armor.
    pub fn unequip_armor(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.check_unequip_armor()?;
        self.exaltation.unequip_armor()?;
        Ok(self)
    }
}
