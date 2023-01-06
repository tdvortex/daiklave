use crate::{
    book_reference::{BookReference, Book}, exaltation::exalt::essence::MoteCommitment,
    hearthstone::OwnedHearthstone,
};

use super::{
    artifact::ArtifactWeapon, base::BaseWeaponMemo, equipped::Equipped, mundane::MundaneWeapon,
    ArtifactWeaponId, BaseWeaponId, WeaponId, WeaponWeightClass, WeaponTag,
};

pub(crate) enum WeaponType<'source> {
    Unarmed,
    Mundane(BaseWeaponId, MundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, ArtifactWeapon<'source>, Option<u8>),
}

impl<'view, 'source> WeaponType<'source> {
    pub fn id(&self) -> WeaponId {
        match self {
            WeaponType::Mundane(base_id, _) => WeaponId::Mundane(*base_id),
            WeaponType::Artifact(artifact_id, _, _) => WeaponId::Artifact(*artifact_id),
            WeaponType::Unarmed => WeaponId::Unarmed,
        }
    }

    pub fn is_artifact(&self) -> bool {
        matches!(self, WeaponType::Artifact(_, _, _))
    }

    pub fn is_attuned(&self) -> bool {
        match self {
            WeaponType::Mundane(_, _) | WeaponType::Unarmed => false,
            WeaponType::Artifact(_, _, maybe) => maybe.is_some(),
        }
    }

    pub fn is_equipped(&self) -> Option<Equipped> {
        match self {
            WeaponType::Mundane(_, mundane) => mundane.is_equipped(),
            WeaponType::Artifact(_, artifact, _) => artifact.is_equipped(),
            WeaponType::Unarmed => Some(Equipped::Natural),
        }
    }

    pub fn mote_commitment(&self) -> Option<(ArtifactWeaponId, MoteCommitment<'source>)> {
        match self {
            WeaponType::Mundane(_, _) | WeaponType::Unarmed=> None,
            WeaponType::Artifact(_, _, _) => todo!(),
        }
    }

    pub fn name(&'view self) -> &'source str {
        match self {
            WeaponType::Mundane(_, mundane) => mundane.name(),
            WeaponType::Artifact(_, artifact, _) => artifact.name(),
            WeaponType::Unarmed => "Unarmed",
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            WeaponType::Mundane(_, mundane) => mundane.book_reference,
            WeaponType::Artifact(_, artifact, _) => artifact.book_reference,
            WeaponType::Unarmed => Some(BookReference::new(Book::CoreRulebook, 582)),
        }
    }

    pub fn lore(&'view self) -> Option<&'source str> {
        match self {
            WeaponType::Mundane(_, _) | WeaponType::Unarmed => None,
            WeaponType::Artifact(_, artifact, _) => artifact.lore(),
        }
    }

    pub fn powers(&self) -> Option<&'source str> {
        match self {
            WeaponType::Mundane(_, _) | WeaponType::Unarmed=> None,
            WeaponType::Artifact(_, artifact, _) => artifact.powers(),
        }
    }

    pub fn hearthstone_slots(&self) -> u8 {
        match self {
            WeaponType::Mundane(_, _) | WeaponType::Unarmed => 0,
            WeaponType::Artifact(_, artifact, _) => {
                artifact.hearthstone_slots.len().min(u8::MAX as usize) as u8
            }
        }
    }

    pub fn slotted_hearthstones(
        &'view self,
    ) -> impl Iterator<Item = &'view OwnedHearthstone<'source>> {
        match self {
            WeaponType::Mundane(_, _) | WeaponType::Unarmed => Vec::new().into_iter(),
            WeaponType::Artifact(_, artifact, _) => (**artifact)
                .slotted_hearthstones()
                .collect::<Vec<&'view OwnedHearthstone<'source>>>()
                .into_iter(),
        }
    }

    pub fn base_artifact_weapon(&self) -> Option<(BaseWeaponId, &'source BaseWeaponMemo)> {
        match self {
            WeaponType::Mundane(_, _) | WeaponType::Unarmed => None,
            WeaponType::Artifact(_, artifact, _) => Some((
                artifact.base_artifact_weapon_id(),
                artifact.base_artifact_weapon(),
            )),
        }
    }

    pub fn weight_class(&self) -> WeaponWeightClass {
        match self {
            WeaponType::Unarmed => WeaponWeightClass::Light,
            WeaponType::Mundane(_, mundane) => mundane.weight_class,
            WeaponType::Artifact(_, artifact, _) => artifact.base_artifact_weapon().weight_class,
        }
    }

    pub fn tags(&self) -> impl Iterator<Item = WeaponTag> + '_ {
        match self {
            WeaponType::Unarmed => vec![
                WeaponTag::Bashing,
                WeaponTag::Brawl,
                WeaponTag::Grappling,
                WeaponTag::Natural,
            ].into_iter(),
            WeaponType::Mundane(_, mundane) => mundane.tags(),
            WeaponType::Artifact(_, artifact, _) => artifact.tags(),
        }
    }
}
