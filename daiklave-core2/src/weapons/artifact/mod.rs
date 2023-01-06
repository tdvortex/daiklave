mod handless;
mod named;
mod natural;
mod nonnatural;
mod one_handed;
mod two_handed;
mod worn;

use std::ops::Deref;

pub(in crate::weapons) use handless::{
    HandlessArtifactWeapon, HandlessArtifactWeaponMemo, HandlessArtifactWeaponNoAttunement,
    HandlessArtifactWeaponNoAttunementMemo,
};
pub(in crate::weapons) use natural::{NaturalArtifactWeapon, NaturalArtifactWeaponMemo};
pub(in crate::weapons) use nonnatural::{
    NonnaturalArtifactWeapon, NonnaturalArtifactWeaponMemo, NonnaturalArtifactWeaponNoAttunement,
    NonnaturalArtifactWeaponNoAttunementMemo,
};
pub(in crate::weapons) use one_handed::{OneHandedArtifactWeapon, OneHandedArtifactWeaponMemo};
pub(in crate::weapons) use two_handed::{TwoHandedArtifactWeapon, TwoHandedArtifactWeaponMemo};
pub(in crate::weapons) use worn::{WornArtifactWeapon, WornArtifactWeaponMemo};


use self::named::NamedArtifactWeapon;
pub(crate) use self::named::NamedArtifactWeaponMemo;

use super::{EquipHand, Equipped};

/// An artifact weapon, discriminated by its wielding characteristics.
pub enum ArtifactWeapon<'source> {
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

impl<'view, 'source> Deref for ArtifactWeapon<'source> {
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
    pub fn name(&self) -> &'source str {
        match self {
            ArtifactWeapon::Natural(weapon) => weapon.name(),
            ArtifactWeapon::Worn(weapon, _) => weapon.name(),
            ArtifactWeapon::OneHanded(weapon, _) => weapon.name(),
            ArtifactWeapon::TwoHanded(weapon, _) => weapon.name(),
        }
    }

    pub fn lore(&self) -> Option<&'source str> {
        match self {
            ArtifactWeapon::Natural(weapon) => weapon.lore(),
            ArtifactWeapon::Worn(weapon, _) => weapon.lore(),
            ArtifactWeapon::OneHanded(weapon, _) => weapon.lore(),
            ArtifactWeapon::TwoHanded(weapon, _) => weapon.lore(),
        }
    }

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

    /// Creates (by cloning) an owned copy of the artifact weapon.
    pub fn as_memo(&self) -> ArtifactWeaponMemo {
        match self {
            ArtifactWeapon::Natural(view) => ArtifactWeaponMemo::Natural(view.as_memo()),
            ArtifactWeapon::Worn(view, equipped) => ArtifactWeaponMemo::Worn(view.as_memo(), *equipped),
            ArtifactWeapon::OneHanded(view, equipped) => ArtifactWeaponMemo::OneHanded(view.as_memo(), *equipped),
            ArtifactWeapon::TwoHanded(view, equipped) => ArtifactWeaponMemo::TwoHanded(view.as_memo(), *equipped),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactWeaponMemo {
    Natural(NaturalArtifactWeaponMemo),
    Worn(WornArtifactWeaponMemo, bool),
    OneHanded(OneHandedArtifactWeaponMemo, Option<EquipHand>),
    TwoHanded(TwoHandedArtifactWeaponMemo, bool),
}

impl<'source> ArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> ArtifactWeapon<'source> {
        match self {
            ArtifactWeaponMemo::Natural(memo) => ArtifactWeapon::Natural(memo.as_ref()),
            ArtifactWeaponMemo::Worn(memo, equipped) => ArtifactWeapon::Worn(memo.as_ref(), *equipped),
            ArtifactWeaponMemo::OneHanded(memo, equipped) => ArtifactWeapon::OneHanded(memo.as_ref(), *equipped),
            ArtifactWeaponMemo::TwoHanded(memo, equipped) => ArtifactWeapon::TwoHanded(memo.as_ref(), *equipped),
        }
    }
}