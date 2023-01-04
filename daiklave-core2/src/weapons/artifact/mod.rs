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
pub(in crate::weapons) use base::{BaseArtifactWeapon, BaseArtifactWeaponMemo};

use self::named::NamedArtifactWeapon;

use super::{EquipHand, Equipped};

pub enum ArtifactWeapon<'view, 'source> {
    Natural(&'view NaturalArtifactWeapon<'source>),
    Worn(&'view WornArtifactWeapon<'source>, bool),
    OneHanded(&'view OneHandedArtifactWeapon<'source>, Option<EquipHand>),
    TwoHanded(&'view TwoHandedArtifactWeapon<'source>, bool),
}

impl<'view, 'source> Deref for ArtifactWeapon<'view, 'source> {
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

impl<'view, 'source> ArtifactWeapon<'view, 'source> {
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