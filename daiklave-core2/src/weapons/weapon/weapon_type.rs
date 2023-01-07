use crate::{
    book_reference::{Book, BookReference},
    exaltation::exalt::essence::MoteCommitment,
    hearthstone::OwnedHearthstone,
};

use super::{
    artifact::ArtifactWeapon, base::BaseWeapon, equipped::Equipped, mundane::MundaneWeapon,
    ArtifactWeaponId, AttackRange, BaseWeaponId, WeaponId, WeaponTag, WeaponWeightClass,
};

pub(crate) enum WeaponType<'source> {
    Unarmed,
    Mundane(BaseWeaponId, MundaneWeapon<'source>, u8),
    Artifact(ArtifactWeaponId, ArtifactWeapon<'source>, Option<u8>),
}

impl<'view, 'source> WeaponType<'source> {
    pub fn id(&self) -> WeaponId {
        match self {
            WeaponType::Mundane(base_id, _, _) => WeaponId::Mundane(*base_id),
            WeaponType::Artifact(artifact_id, _, _) => WeaponId::Artifact(*artifact_id),
            WeaponType::Unarmed => WeaponId::Unarmed,
        }
    }

    pub fn is_artifact(&self) -> bool {
        matches!(self, WeaponType::Artifact(_, _, _))
    }

    pub fn is_attuned(&self) -> bool {
        match self {
            WeaponType::Mundane(_, _, _) | WeaponType::Unarmed => false,
            WeaponType::Artifact(_, _, maybe) => maybe.is_some(),
        }
    }

    pub fn is_equipped(&self) -> Option<Equipped> {
        match self {
            WeaponType::Mundane(_, mundane, _) => mundane.is_equipped(),
            WeaponType::Artifact(_, artifact, _) => artifact.is_equipped(),
            WeaponType::Unarmed => Some(Equipped::Natural),
        }
    }

    pub fn mote_commitment(&self) -> Option<(ArtifactWeaponId, MoteCommitment<'source>)> {
        match self {
            WeaponType::Mundane(_, _, _) | WeaponType::Unarmed => None,
            WeaponType::Artifact(artifact_weapon_id, named_artifact, maybe_personal) => {
                let personal = *maybe_personal.as_ref()?;
                let peripheral = 5 - 5.min(personal);
                let name = named_artifact.name();
                Some((
                    *artifact_weapon_id,
                    MoteCommitment {
                        name,
                        peripheral,
                        personal,
                    },
                ))
            }
        }
    }

    pub fn name(&'view self) -> &'source str {
        match self {
            WeaponType::Mundane(_, mundane, _) => mundane.name(),
            WeaponType::Artifact(_, artifact, _) => artifact.name(),
            WeaponType::Unarmed => "Unarmed",
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            WeaponType::Mundane(_, mundane, _) => mundane.book_reference,
            WeaponType::Artifact(_, artifact, _) => artifact.book_reference,
            WeaponType::Unarmed => Some(BookReference::new(Book::CoreRulebook, 582)),
        }
    }

    pub fn lore(&'view self) -> Option<&'source str> {
        match self {
            WeaponType::Mundane(_, _, _) | WeaponType::Unarmed => None,
            WeaponType::Artifact(_, artifact, _) => artifact.lore(),
        }
    }

    pub fn powers(&self) -> Option<&'source str> {
        match self {
            WeaponType::Mundane(_, _, _) | WeaponType::Unarmed => None,
            WeaponType::Artifact(_, artifact, _) => artifact.powers(),
        }
    }

    pub fn hearthstone_slots(&self) -> u8 {
        match self {
            WeaponType::Mundane(_, _, _) | WeaponType::Unarmed => 0,
            WeaponType::Artifact(_, artifact, _) => {
                artifact.hearthstone_slots.len().min(u8::MAX as usize) as u8
            }
        }
    }

    pub fn slotted_hearthstones(
        &'view self,
    ) -> impl Iterator<Item = &'view OwnedHearthstone<'source>> {
        match self {
            WeaponType::Mundane(_, _, _) | WeaponType::Unarmed => Vec::new().into_iter(),
            WeaponType::Artifact(_, artifact, _) => (**artifact)
                .slotted_hearthstones()
                .collect::<Vec<&'view OwnedHearthstone<'source>>>()
                .into_iter(),
        }
    }

    pub fn base_artifact_weapon(&self) -> Option<(BaseWeaponId, &'source BaseWeapon)> {
        match self {
            WeaponType::Mundane(_, _, _) | WeaponType::Unarmed => None,
            WeaponType::Artifact(_, artifact, _) => Some((
                artifact.base_artifact_weapon_id(),
                artifact.base_artifact_weapon(),
            )),
        }
    }

    pub fn weight_class(&self) -> WeaponWeightClass {
        match self {
            WeaponType::Unarmed => WeaponWeightClass::Light,
            WeaponType::Mundane(_, mundane, _) => mundane.weight_class,
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
            ]
            .into_iter(),
            WeaponType::Mundane(_, mundane, _) => mundane.tags(),
            WeaponType::Artifact(_, artifact, _) => artifact.tags(),
        }
    }

    pub fn accuracy(&self, attack_range: AttackRange) -> Option<i8> {
        match self {
            WeaponType::Unarmed => {
                if attack_range == AttackRange::Melee {
                    Some(4)
                } else {
                    None
                }
            }
            WeaponType::Mundane(_, mundane, _) => mundane.accuracy(attack_range, false),
            WeaponType::Artifact(_, artifact, _) => {
                artifact.base_artifact_weapon().accuracy(attack_range, true)
            }
        }
    }

    pub fn damage(&self, attack_range: AttackRange) -> Option<u8> {
        match self {
            WeaponType::Unarmed => {
                if attack_range == AttackRange::Melee {
                    Some(7)
                } else {
                    None
                }
            }
            WeaponType::Mundane(_, mundane, _) => mundane.damage(attack_range, false),
            WeaponType::Artifact(_, artifact, _) => {
                artifact.base_artifact_weapon().damage(attack_range, true)
            }
        }
    }

    pub fn parry_mod(&self) -> Option<i8> {
        match self {
            WeaponType::Unarmed => Some(0),
            WeaponType::Mundane(_, mundane, _) => mundane.parry_mod(false),
            WeaponType::Artifact(_, artifact, _) => artifact.base_artifact_weapon().parry_mod(true),
        }
    }

    pub fn overwhelming(&self) -> u8 {
        match self {
            WeaponType::Unarmed => 1,
            WeaponType::Mundane(_, mundane, _) => mundane.overwhelming(false),
            WeaponType::Artifact(_, artifact, _) => {
                artifact.base_artifact_weapon().overwhelming(true)
            }
        }
    }

    pub fn is_natural(&self) -> bool {
        match self {
            WeaponType::Unarmed => true,
            WeaponType::Mundane(_, mundane, _) => matches!(mundane, MundaneWeapon::Natural(_)),
            WeaponType::Artifact(_, artifact, _) => matches!(artifact, ArtifactWeapon::Natural(_)),
        }
    }

    pub fn is_worn(&self) -> bool {
        match self {
            WeaponType::Unarmed => false,
            WeaponType::Mundane(_, mundane, _) => matches!(mundane, MundaneWeapon::Worn(..)),
            WeaponType::Artifact(_, artifact, _) => matches!(artifact, ArtifactWeapon::Worn(..)),
        }
    }

    pub fn is_one_handed(&self) -> bool {
        match self {
            WeaponType::Unarmed => false,
            WeaponType::Mundane(_, mundane, _) => matches!(mundane, MundaneWeapon::OneHanded(..)),
            WeaponType::Artifact(_, artifact, _) => {
                matches!(artifact, ArtifactWeapon::OneHanded(..))
            }
        }
    }

    pub fn is_two_handed(&self) -> bool {
        match self {
            WeaponType::Unarmed => false,
            WeaponType::Mundane(_, mundane, _) => matches!(mundane, MundaneWeapon::TwoHanded(..)),
            WeaponType::Artifact(_, artifact, _) => {
                matches!(artifact, ArtifactWeapon::TwoHanded(..))
            }
        }
    }

    pub fn quantity(&self) -> u8 {
        match self {
            WeaponType::Unarmed => 1,
            WeaponType::Mundane(_, _, count) => *count,
            WeaponType::Artifact(_, _, _) => 1,
        }
    }
}
