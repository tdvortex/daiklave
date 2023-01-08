use thiserror::Error;

/// An error related to armor
#[derive(Debug, Error)]
pub enum ArmorError {
    /// Can't equip an already equipped piece of armor.
    #[error("The armor item is already equipped and cannot be reequipped")]
    AlreadyEquipped,
    /// Armor (even mundane armor) is unique.
    #[error("Armor items must be unique")]
    DuplicateArmor,
    /// Can't equip or remove an armor item if it doesn't exist. Also, can't
    /// unequip armor if nothing is equipped.
    #[error("Armor item Id not found")]
    NotFound,
    /// Can't remove an item that is currently equipped; unequip it first
    #[error("Armor must be unequipped before it can be removed")]
    RemoveEquipped,
}
