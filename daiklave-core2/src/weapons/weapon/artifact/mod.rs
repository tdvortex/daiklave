mod base;
/// A sequential set of builder types to construct a new artifact weapon.
pub mod builder;
mod grouped;
mod id;
mod memo;
mod named;
mod newtype;

use std::ops::Deref;

pub use base::BaseArtifactWeapon;
pub(crate) use grouped::{
    HandlessArtifactWeapon, HandlessArtifactWeaponMemo, HandlessArtifactWeaponNoAttunement,
    HandlessArtifactWeaponNoAttunementMemo, NonnaturalArtifactWeapon, NonnaturalArtifactWeaponMemo,
    NonnaturalArtifactWeaponNoAttunement, NonnaturalArtifactWeaponNoAttunementMemo,
};
pub use id::ArtifactWeaponId;
pub(crate) use memo::ArtifactWeaponMemo;
pub use newtype::{
    NaturalArtifactWeapon, NaturalArtifactWeaponMemo, OneHandedArtifactWeapon,
    OneHandedArtifactWeaponMemo, TwoHandedArtifactWeapon, TwoHandedArtifactWeaponMemo,
    WornArtifactWeapon, WornArtifactWeaponMemo,
};

use self::named::NamedArtifactWeapon;

use super::{
    equipped::{EquipHand, Equipped},
    WeaponTag,
};

/// An artifact weapon, discriminated by its wielding characteristics.
pub(crate) enum ArtifactWeapon<'source> {
    /// An artifact weapon that is part of the wielder's body. This is unusual,
    /// but possible through Charm effects like Silver-Voiced Nightengale
    /// Style, the Blood Lash spell, or certain Lunar shapeshifting Charms.
    Natural(NaturalArtifactWeapon<'source>),
    /// An artifact weapon that is worn and does not occupy a hand, such as
    /// Smashfists or Razor Claws. The second parameter indicates whether it
    /// is currently equipped.
    Worn(WornArtifactWeapon<'source>, bool),
    /// A one-handed artifact weapon. If it is equipped, the second parameter
    /// is Some with the hand it occupies.
    OneHanded(OneHandedArtifactWeapon<'source>, Option<EquipHand>),
    /// A two-handed artifact weapon. The second parameter indicates whether it
    /// is currently equipped.
    TwoHanded(TwoHandedArtifactWeapon<'source>, bool),
}

impl<'source> Deref for ArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        match self {
            ArtifactWeapon::Natural(deref) => deref,
            ArtifactWeapon::Worn(deref, _) => deref,
            ArtifactWeapon::OneHanded(deref, _) => deref,
            ArtifactWeapon::TwoHanded(deref, _) => deref,
        }
    }
}

impl<'source> ArtifactWeapon<'source> {
    /// Returns the name of the weapon. This is the unique name (like "Volcano
    /// Cutter").
    pub fn name(&self) -> &'source str {
        match self {
            ArtifactWeapon::Natural(weapon) => weapon.name(),
            ArtifactWeapon::Worn(weapon, _) => weapon.name(),
            ArtifactWeapon::OneHanded(weapon, _) => weapon.name(),
            ArtifactWeapon::TwoHanded(weapon, _) => weapon.name(),
        }
    }

    /// If present, returns the lore text (forging, prior wielders, etc.) of
    /// the weapon.
    pub fn lore(&self) -> Option<&'source str> {
        match self {
            ArtifactWeapon::Natural(weapon) => weapon.lore(),
            ArtifactWeapon::Worn(weapon, _) => weapon.lore(),
            ArtifactWeapon::OneHanded(weapon, _) => weapon.lore(),
            ArtifactWeapon::TwoHanded(weapon, _) => weapon.lore(),
        }
    }

    /// If present, returns the special non-Evocation powers of the weapon.
    pub fn powers(&self) -> Option<&'source str> {
        match self {
            ArtifactWeapon::Natural(weapon) => weapon.powers(),
            ArtifactWeapon::Worn(weapon, _) => weapon.powers(),
            ArtifactWeapon::OneHanded(weapon, _) => weapon.powers(),
            ArtifactWeapon::TwoHanded(weapon, _) => weapon.powers(),
        }
    }

    /// Returns None if not equipped, or an enum representing its equipped
    /// status (Natural, Worn, TwoHanded, MainHand, or OffHand).
    pub fn is_equipped(&self) -> Option<Equipped> {
        match self {
            ArtifactWeapon::Natural(_) => Some(Equipped::Natural),
            ArtifactWeapon::Worn(_, is_equipped) => {
                if *is_equipped {
                    Some(Equipped::Worn)
                } else {
                    None
                }
            }
            ArtifactWeapon::OneHanded(_, maybe_hand) => match maybe_hand {
                None => None,
                Some(EquipHand::MainHand) => Some(Equipped::MainHand),
                Some(EquipHand::OffHand) => Some(Equipped::OffHand),
            },
            ArtifactWeapon::TwoHanded(_, is_equipped) => {
                if *is_equipped {
                    Some(Equipped::TwoHanded)
                } else {
                    None
                }
            }
        }
    }

    pub(crate) fn tags(&self) -> std::vec::IntoIter<WeaponTag> {
        match self {
            ArtifactWeapon::Natural(base) => base.base_artifact_weapon().tags(WeaponTag::Natural),
            ArtifactWeapon::Worn(base, _) => base.base_artifact_weapon().tags(WeaponTag::Worn),
            ArtifactWeapon::OneHanded(base, _) => {
                base.base_artifact_weapon().tags(WeaponTag::OneHanded)
            }
            ArtifactWeapon::TwoHanded(base, _) => {
                base.base_artifact_weapon().tags(WeaponTag::TwoHanded)
            }
        }
    }
}
