use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArmorError {
    #[error("The armor item is already equipped and cannot be reequipped")]
    AlreadyEquipped,
    #[error("Armor items must be unique")]
    DuplicateArmor,
    #[error("Armor item Id not found")]
    NotFound,
    #[error("Armor must be unequipped before it can be removed")]
    RemoveEquipped,
}