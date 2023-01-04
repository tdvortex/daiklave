mod base;
mod handless;
mod named;
mod natural;
mod nonnatural;
mod one_handed;
mod two_handed;
mod worn;

use std::ops::Deref;

pub(in crate::weapons) use handless::{HandlessArtifactWeapon, HandlessArtifactWeaponNoAttunement, HandlessArtifactWeaponMemo, HandlessArtifactWeaponNoAttunementMemo};
pub use one_handed::{OneHandedArtifactWeapon, OneHandedArtifactWeaponMemo};
pub use two_handed::{TwoHandedArtifactWeapon, TwoHandedArtifactWeaponMemo};
pub(in crate::weapons) use nonnatural::{NonnaturalArtifactWeapon, NonnaturalArtifactWeaponNoAttunement, NonnaturalArtifactWeaponMemo, NonnaturalArtifactWeaponNoAttunementMemo};
pub use natural::{NaturalArtifactWeapon, NaturalArtifactWeaponMemo};
pub use worn::{WornArtifactWeapon, WornArtifactWeaponMemo};
pub use base::{BaseArtifactWeapon, BaseArtifactWeaponMemo};

use self::named::NamedArtifactWeapon;

use super::{EquipHand, Equipped};

pub enum ArtifactWeapon<'source> {
    Natural(NaturalArtifactWeapon<'source>),
    Worn(WornArtifactWeapon<'source>, bool),
    OneHanded(OneHandedArtifactWeapon<'source>, Option<EquipHand>),
    TwoHanded(TwoHandedArtifactWeapon<'source>, bool),
}

impl<'source> Deref for ArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        match self {
            ArtifactWeapon::Natural(deref) => deref,
            ArtifactWeapon::Worn(deref, is_worn) => deref,
            ArtifactWeapon::OneHanded(deref, maybe_hand) => deref,
            ArtifactWeapon::TwoHanded(deref, is_equipped) => deref,
        }
    }
}

impl<'source> ArtifactWeapon<'source> {
    pub fn is_equipped(&self) -> Option<Equipped> {
        match self {
            ArtifactWeapon::Natural(_) => Some(Equipped::Natural),
            ArtifactWeapon::Worn(_, is_equipped) => if *is_equipped {
                Some(Equipped::Worn)
            } else {
                None
            },
            ArtifactWeapon::OneHanded(_, maybe_hand) => match maybe_hand {
                None => None,
                Some(EquipHand::MainHand) => Some(Equipped::MainHand),
                Some(EquipHand::OffHand) => Some(Equipped::OffHand),
            }
            ArtifactWeapon::TwoHanded(_, is_equipped) => if *is_equipped {
                Some(Equipped::TwoHanded)
            } else {
                None
            }, 
        }
    }
}