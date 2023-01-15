pub mod merit;

use crate::{
    armor::armor_item::ArmorId, artifact::ArtifactId, exaltation::Exaltation,
    weapons::weapon::WeaponId, Character,
};

use self::merit::{Merit, MeritId, MeritSource};

/// The merits possessed by a character.
pub struct Merits<'view, 'source>(pub(crate) &'view Character<'source>);

impl<'view, 'source> Merits<'view, 'source> {
    /// Gets a specific Merit belonging to the character (if it exists).
    pub fn get(&self, merit_id: MeritId) -> Option<Merit<'source>> {
        match merit_id {
            MeritId::Artifact(artifact_id) => match artifact_id {
                ArtifactId::Weapon(artifact_weapon_id) => self
                    .0
                    .weapons()
                    .iter()
                    .find_map(|(weapon_id, equipped)| {
                        if weapon_id == WeaponId::Artifact(artifact_weapon_id) {
                            self.0.weapons().get(weapon_id, equipped)
                        } else {
                            None
                        }
                    })
                    .and_then(|weapon| {
                        weapon.merit_dots().map(|dots| {
                            Merit(MeritSource::Artifact(
                                ArtifactId::Weapon(artifact_weapon_id),
                                weapon.name(),
                                dots,
                            ))
                        })
                    }),
                ArtifactId::Armor(artifact_armor_id) => self
                    .0
                    .armor()
                    .get(ArmorId::Artifact(artifact_armor_id))
                    .and_then(|armor| {
                        armor.merit_dots().map(|dots| {
                            Merit(MeritSource::Artifact(
                                ArtifactId::Armor(artifact_armor_id),
                                armor.name(),
                                dots,
                            ))
                        })
                    }),
                ArtifactId::Wonder(wonder_id) => self.0.wonders().get(wonder_id).map(|wonder| {
                    Merit(MeritSource::Artifact(
                        ArtifactId::Wonder(wonder_id),
                        wonder.name(),
                        wonder.merit_dots(),
                    ))
                }),
            },
            MeritId::DemenseNoManse(demense_id) => {
                self.0
                    .demenses_no_manse
                    .get(&demense_id)
                    .map(|(name, geomancy)| {
                        Merit(MeritSource::DemenseNoManse(demense_id, *name, *geomancy))
                    })
            }
            MeritId::DemenseWithManse(hearthstone_id) => self
                .0
                .hearthstones()
                .get(hearthstone_id)
                .and_then(|hearthstone| {
                    hearthstone.manse_and_demense().map(|(_, demense)| {
                        Merit(MeritSource::DemenseWithManse(
                            hearthstone_id,
                            demense,
                            hearthstone.geomancy_level(),
                        ))
                    })
                }),
            MeritId::ExaltedHealing => {
                match &self.0.exaltation {
                    Exaltation::Mortal(mortal) => {
                        if mortal.exalted_healing {
                            Some(Merit(MeritSource::ExaltedHealing(false)))
                        } else {
                            None
                        }
                    }
                    Exaltation::Exalt(_) => Some(Merit(MeritSource::ExaltedHealing(true))),
                }
            }
            MeritId::HearthstoneNoManse(hearthstone_id) => {
                self.0
                .hearthstones()
                .get(hearthstone_id)
                .and_then(|hearthstone| {
                    if hearthstone.manse_and_demense().is_some() {
                        None
                    } else {
                        Some(Merit(MeritSource::HearthstoneNoManse(hearthstone_id, hearthstone.name(), hearthstone.geomancy_level())))
                    }
                })
            }
            MeritId::HearthstoneWithManse(hearthstone_id) => {
                self.0
                .hearthstones()
                .get(hearthstone_id)
                .and_then(|hearthstone| {
                    hearthstone.manse_and_demense().map(|_| Merit(MeritSource::HearthstoneWithManse(hearthstone_id, hearthstone.name(), hearthstone.geomancy_level())))
                })
            }
            MeritId::Manse(hearthstone_id) => {
                self.0
                    .hearthstones()
                    .get(hearthstone_id)
                    .and_then(|hearthstone| {
                        hearthstone.manse_and_demense().map(|(manse, _)| {
                            Merit(MeritSource::Manse(
                                hearthstone_id,
                                manse,
                                hearthstone.geomancy_level(),
                            ))
                        })
                    })
            }
            MeritId::MartialArtist(style_id) => self
                .0
                .martial_arts()
                .style(style_id)
                .map(|style| Merit(MeritSource::MartialArtist(style_id, style.name()))),
            MeritId::MortalSorcerer => {
                if let Exaltation::Mortal(mortal) = &self.0.exaltation {
                    if mortal.sorcery.is_some() {
                        Some(Merit(MeritSource::MortalSorcerer))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            MeritId::NonStackableMerit(nonstackable_id) => self
                .0
                .nonstackable_merits
                .get(&nonstackable_id)
                .map(|merit| {
                    Merit(MeritSource::NonStackableMerit(
                        nonstackable_id,
                        merit.clone(),
                    ))
                }),
            MeritId::StackableMerit(stackable_id) => self
                .0
                .stackable_merits
                .get(&stackable_id)
                .map(|merit| Merit(MeritSource::StackableMerit(stackable_id, *merit))),
            MeritId::LocalTongues => todo!(),
            MeritId::MajorLanguage(_) => todo!(),
        }
    }

    /// Iterates over all Merits owned by the character by their Id.
    pub fn iter(&self) -> impl Iterator<Item = MeritId> + '_ {
        vec![].into_iter()
    }
}
