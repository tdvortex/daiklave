mod error;
mod mote_commitment;
mod mote_pool;
mod motes;
mod set_rating;
mod state;
pub(crate) use state::{EssenceState, EssenceStateMemo};

pub(crate) use error::EssenceError;
pub(crate) use motes::MotesState;
pub use motes::{CommitMotes, Motes, RecoverMotes, SpendMotes, UncommitMotes};

use crate::{
    armor::armor_item::{ArmorType, ArmorWeightClass},
    artifact::ArtifactName,
    weapons::weapon::WeaponType,
};

use super::{AnimaEffect, Exalt};
pub(crate) use mote_commitment::MoteCommitmentNameMutation;
pub use mote_commitment::{MoteCommitment, MoteCommitmentName};
pub(crate) use mote_pool::MotePool;
pub use mote_pool::MotePoolName;
pub use set_rating::SetEssenceRating;

/// An Exalt's Essence rating and mote pools.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Essence<'view, 'source>(pub(crate) &'view Exalt<'source>);

impl<'view, 'source> Essence<'view, 'source> {
    /// The Exalt's current Essence rating.
    pub fn rating(&self) -> u8 {
        self.0.essence.rating.get()
    }

    /// The current state of the Exalt's mote pools.
    pub fn motes(&self) -> Motes<'view, 'source> {
        let weapon_attunements =
            self.0.weapons.iter().filter_map(|(weapon_id, equipped)| {
                match self
                    .0
                    .weapons
                    .get_weapon(weapon_id, equipped)
                    .map(|weapon| weapon.0)
                {
                    None => None,
                    Some(WeaponType::Unarmed) => None,
                    Some(WeaponType::Mundane(_, _, _)) => None,
                    Some(WeaponType::Artifact(artifact_weapon_name, _weapon, attunement)) => {
                        attunement.map(|personal| MoteCommitment {
                            name: MoteCommitmentName::AttunedArtifact(ArtifactName::Weapon(
                                artifact_weapon_name,
                            )),
                            peripheral: 5 - personal.min(5),
                            personal: personal.min(5),
                        })
                    }
                }
            });

        let armor_attunements = self.0.armor.iter().filter_map(|armor_id| {
            self.0
                .armor
                .get(armor_id)
                .and_then(|armor_item| match armor_item.0 {
                    ArmorType::Artifact(artifact_armor_name, armor, attunement) => {
                        if let Some(personal) = attunement {
                            let required = match armor.base_armor().weight_class() {
                                ArmorWeightClass::Light => 4,
                                ArmorWeightClass::Medium => 5,
                                ArmorWeightClass::Heavy => 6,
                            };

                            Some(MoteCommitment {
                                name: MoteCommitmentName::AttunedArtifact(ArtifactName::Armor(
                                    artifact_armor_name,
                                )),
                                peripheral: required - personal.min(required),
                                personal: personal.min(required),
                            })
                        } else {
                            None
                        }
                    }
                    ArmorType::Mundane(_, _) => None,
                })
        });

        let wonder_attunements = self.0.wonders.iter().filter_map(|wonder_name| {
            self.0.wonders.get(wonder_name).and_then(|wonder| {
                if let Some(personal) = wonder.2 {
                    if let Some(required) = wonder.1.attunement_cost {
                        Some(MoteCommitment {
                            name: MoteCommitmentName::AttunedArtifact(ArtifactName::Wonder(
                                wonder.0,
                            )),
                            peripheral: required - personal.min(required),
                            personal: personal.min(required),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        });

        let attunements = weapon_attunements
            .chain(armor_attunements)
            .chain(wonder_attunements)
            .collect();

        Motes {
            state: &self.0.essence.motes,
            attunements,
        }
    }

    /// The anima effects the Exalt possesses.
    pub fn anima_effects(&self) -> impl Iterator<Item = AnimaEffect> {
        self.0.anima_effects()
    }
}
