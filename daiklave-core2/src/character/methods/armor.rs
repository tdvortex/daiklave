use crate::{
    armor::{
        armor_item::{mundane::MundaneArmor, ArmorId, BaseArmorId},
        Armor,
    },
    Character, CharacterMutationError,
};

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
        self.exaltation.add_mundane_armor(armor_id, armor)?;
        Ok(self)
    }

    /// Removes a piece of mundane armor from a character.
    pub fn remove_mundane_armor(
        &mut self,
        armor_id: BaseArmorId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.remove_mundane_armor(armor_id)?;
        Ok(self)
    }

    /// Equips a specific piece of armor to a character.
    pub fn equip_armor(&mut self, armor_id: ArmorId) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.equip_armor(armor_id)?;
        Ok(self)
    }

    /// Unequips the currently-equipped piece of armor.
    pub fn unequip_armor(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.unequip_armor()?;
        Ok(self)
    }
}
