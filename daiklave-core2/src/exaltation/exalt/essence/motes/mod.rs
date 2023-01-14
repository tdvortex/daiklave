use crate::{
    armor::armor_item::{ArmorType, ArmorWeightClass},
    artifact::ArtifactId,
    exaltation::exalt::{ExaltArmor, ExaltWeapons, ExaltWonders},
    weapons::weapon::WeaponType,
};

mod state;
pub(crate) use state::{MotesState, MotesStateMemo};

use super::{MoteCommitment, MoteCommitmentId, MotePool};

/// The current status of an Exalt's motes of Essence.
pub struct Motes<'view, 'source> {
    pub(crate) state: &'view MotesState<'source>,
    pub(crate) weapons: &'view ExaltWeapons<'source>,
    pub(crate) armor: &'view ExaltArmor<'source>,
    pub(crate) wonders: &'view ExaltWonders<'source>,
}

impl<'view, 'source> Motes<'view, 'source> {
    /// The Exalt's peripheral motes.
    pub fn peripheral(&self) -> &'view MotePool {
        self.state.peripheral()
    }

    /// The Exalt's personal motes.
    pub fn personal(&self) -> &'view MotePool {
        self.state.personal()
    }

    /// All effects the Exalt has currently committed motes to (including
    /// artifact attunement)
    pub fn committed(
        &self,
    ) -> impl Iterator<Item = (MoteCommitmentId, MoteCommitment<'source>)> + '_ {
        let other_commitments = self.state.commitments.iter().map(|(k, v)| (MoteCommitmentId::Other(*k), *v));

        let weapon_commitments =
            self.weapons.iter().filter_map(|(weapon_id, equipped)| {
                match self
                    .weapons
                    .get_weapon(weapon_id, equipped)
                    .map(|weapon| weapon.0)
                {
                    None => None,
                    Some(WeaponType::Unarmed) => None,
                    Some(WeaponType::Mundane(_, _, _)) => None,
                    Some(WeaponType::Artifact(artifact_weapon_id, weapon, attunement)) => {
                        if let Some(personal) = attunement {
                            Some((
                                MoteCommitmentId::AttunedArtifact(ArtifactId::Weapon(
                                    artifact_weapon_id,
                                )),
                                MoteCommitment {
                                    name: weapon.name(),
                                    peripheral: 5 - personal.min(5),
                                    personal: personal.min(5),
                                },
                            ))
                        } else {
                            None
                        }
                    }
                }
            });

        let armor_commitments = self.armor.iter().filter_map(|armor_id| {
            self.armor
                .get(armor_id)
                .and_then(|armor_item| match armor_item.0 {
                    ArmorType::Artifact(artifact_armor_id, armor, attunement) => {
                        if let Some(personal) = attunement {
                            let amount = match armor.base_armor().weight_class() {
                                ArmorWeightClass::Light => 4,
                                ArmorWeightClass::Medium => 5,
                                ArmorWeightClass::Heavy => 6,
                            };

                            Some((
                                MoteCommitmentId::AttunedArtifact(ArtifactId::Armor(
                                    artifact_armor_id,
                                )),
                                MoteCommitment {
                                    name: armor.name(),
                                    peripheral: amount - personal.min(amount),
                                    personal: personal.min(amount),
                                },
                            ))
                        } else {
                            None
                        }
                    }
                    ArmorType::Mundane(_, _) => None,
                })
        });

        let wonder_commitments = self.wonders.iter().filter_map(|wonder_id| {
            self.wonders.get(wonder_id).and_then(|wonder| {
                if let Some(personal) = wonder.2 {
                    if let Some(amount) = wonder.1.attunement_cost {
                        Some((
                            MoteCommitmentId::AttunedArtifact(ArtifactId::Wonder(wonder.0)),
                            MoteCommitment {
                                name: wonder.name(),
                                peripheral: amount - personal.min(amount),
                                personal: personal.min(amount),
                            },
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        });

        other_commitments
            .chain(weapon_commitments)
            .chain(armor_commitments)
            .chain(wonder_commitments)
    }
}
