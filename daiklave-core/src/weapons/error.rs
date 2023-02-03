use thiserror::Error;

/// An error related to character weapons.
#[derive(Debug, Error)]
pub enum WeaponError {
    /// Characters cannot have duplicate natural weapons.
    #[error("Natural weapons must be unique")]
    DuplicateNatural,
    /// Can't wear more than one copy of a Worn weapon (but can wear
    /// multiple different Worn weapons)
    #[error("Only one copy of each worn weapon can be equipped at a time")]
    DuplicateEquippedWorn,
    /// Can't manually equip a Natural weapon
    #[error("Natural weapons are always equipped, cannot be manually re-equipped")]
    EquipNatural,
    /// Have to be specific about hands when equipping one-handed weapons
    #[error("Equipping or unequipping a one-handed weapon requires specifying which hand")]
    HandRequired,
    /// Heavy weapons that are usable in melee require at least Strength 3 to
    /// wield
    #[error("Strength must be at least 3 to wield Heavy melee weapons")]
    HeavyMeleeStrengthRequirement,
    /// Can't add multiple copies of the same artifact
    #[error("Artifacts are unique; cannot own multiple with the same name")]
    DuplicateArtifact,
    /// Trying to equip or remove a missing weapon
    #[error("Weapon not found (or not found at the specified position)")]
    NotFound,
    /// Can't unequip a Natural weapon
    #[error("Natural weapons are always equipped, cannot be unequipped")]
    UnequipNatural,
}
