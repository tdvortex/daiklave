/// Details for an individual merit.
pub mod merit;

use crate::{
    armor::armor_item::ArmorId, artifact::ArtifactId, exaltation::Exaltation,
    languages::language::Language, Character, weapons::weapon::WeaponName,
};

use self::merit::{Merit, MeritId, MeritSource};

/// The merits possessed by a character.
pub struct Merits<'view, 'source>(pub(crate) &'view Character<'source>);

impl<'view, 'source> Merits<'view, 'source> {
    /// Gets a specific Merit belonging to the character (if it exists).
    pub fn get(&self, merit_id: MeritId<'view>) -> Option<Merit<'source>> {
        match merit_id {
            MeritId::Artifact(artifact_id) => match artifact_id {
                ArtifactId::Weapon(search_name) => self
                    .0
                    .weapons()
                    .iter()
                    .find_map(|(source_name, equipped)| {
                        if let WeaponName::Artifact(source_artifact_name) = source_name {
                            if source_artifact_name == search_name {
                                self.0.weapons().get(source_name, equipped).map(|weapon| (source_artifact_name, weapon))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .and_then(|(name, weapon)| {
                        weapon.merit_dots().map(|dots| {
                            Merit(MeritSource::Artifact(
                                ArtifactId::Weapon(name),
                                name,
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
            MeritId::DemenseNoManse(name) => self
                .0
                .demenses_no_manse
                .get_key_value(name)
                .map(|(name, geomancy)| Merit(MeritSource::DemenseNoManse(name, *geomancy))),
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
            MeritId::ExaltedHealing => match &self.0.exaltation {
                Exaltation::Mortal(mortal) => {
                    if mortal.exalted_healing {
                        Some(Merit(MeritSource::ExaltedHealing(false)))
                    } else {
                        None
                    }
                }
                Exaltation::Exalt(_) => Some(Merit(MeritSource::ExaltedHealing(true))),
            },
            MeritId::HearthstoneNoManse(hearthstone_id) => self
                .0
                .hearthstones()
                .get(hearthstone_id)
                .and_then(|hearthstone| {
                    if hearthstone.manse_and_demense().is_some() {
                        None
                    } else {
                        Some(Merit(MeritSource::HearthstoneNoManse(
                            hearthstone_id,
                            hearthstone.name(),
                            hearthstone.geomancy_level(),
                        )))
                    }
                }),
            MeritId::HearthstoneWithManse(hearthstone_id) => self
                .0
                .hearthstones()
                .get(hearthstone_id)
                .and_then(|hearthstone| {
                    hearthstone.manse_and_demense().map(|_| {
                        Merit(MeritSource::HearthstoneWithManse(
                            hearthstone_id,
                            hearthstone.name(),
                            hearthstone.geomancy_level(),
                        ))
                    })
                }),
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
            MeritId::MartialArtist(style_name) => match &self.0.exaltation {
                Exaltation::Mortal(mortal) => mortal
                    .martial_arts_styles
                    .get_key_value(style_name)
                    .map(|(k, _)| Merit(MeritSource::MartialArtist(*k))),
                Exaltation::Exalt(exalt) => exalt
                    .martial_arts_styles
                    .get_key_value(style_name)
                    .map(|(k, _)| Merit(MeritSource::MartialArtist(*k))),
            },
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
            MeritId::NonStackable(nonstackable_id) => self
                .0
                .nonstackable_merits
                .get(&nonstackable_id)
                .map(|merit| Merit(MeritSource::NonStackable(nonstackable_id, merit.clone()))),
            MeritId::Stackable(stackable_id) => self
                .0
                .stackable_merits
                .get(&stackable_id)
                .map(|merit| Merit(MeritSource::Stackable(stackable_id, *merit))),
            MeritId::LocalTongues => {
                let purchased = self
                    .0
                    .languages
                    .iter()
                    .filter(|(language, native)| {
                        !native && matches!(language, Language::LocalTongue(_))
                    })
                    .count();

                if purchased > 0 {
                    Some(Merit(MeritSource::LocalTongues(purchased)))
                } else {
                    None
                }
            }
            MeritId::MajorLanguage(major) => {
                if self
                    .0
                    .languages
                    .other_languages
                    .contains(&Language::MajorLanguage(major))
                {
                    Some(Merit(MeritSource::MajorLanguage(major)))
                } else {
                    None
                }
            }
            MeritId::SorceryArchetype(sorcery_archetype_merit_id) => {
                self.0.sorcery().and_then(|sorcery| {
                    sorcery.archetypes().find_map(|archetype_id| {
                        sorcery.archetype(archetype_id).and_then(|(_, merits)| {
                            merits
                                .get(&sorcery_archetype_merit_id)
                                .map(|sorcery_archetype_merit| {
                                    Merit(MeritSource::SorceryArchetype(
                                        sorcery_archetype_merit_id,
                                        sorcery_archetype_merit,
                                    ))
                                })
                        })
                    })
                })
            }
        }
    }

    /// Iterates over all Merits owned by the character by their Id.
    pub fn iter(&self) -> impl Iterator<Item = MeritId> + '_ {
        // Collect merits Ids into a single vec to minimize heap allocations
        let mut output: Vec<MeritId> = Vec::new();

        // Artifact weapons
        self.0
            .weapons()
            .iter()
            .filter_map(|(weapon_id, equipped)| {
                self.0
                    .weapons()
                    .get(weapon_id, equipped)
                    .and_then(|weapon| {
                        if let WeaponName::Artifact(artifact_weapon_name) = weapon.name() {
                            Some(MeritId::Artifact(ArtifactId::Weapon(artifact_weapon_name)))
                        } else {
                            None
                        }
                    })
            })
            .for_each(|merit_id| output.push(merit_id));

        // Artifact armor
        self.0
            .armor()
            .iter()
            .filter_map(|armor_id| {
                if let ArmorId::Artifact(artifact_armor_id) = armor_id {
                    Some(MeritId::Artifact(ArtifactId::Armor(artifact_armor_id)))
                } else {
                    None
                }
            })
            .for_each(|merit_id| output.push(merit_id));

        // Wonders
        self.0
            .wonders()
            .iter()
            .map(|wonder_id| MeritId::Artifact(ArtifactId::Wonder(wonder_id)))
            .for_each(|merit_id| output.push(merit_id));

        // Demenses without manses
        self.0
            .demenses_no_manse
            .keys()
            .map(|unique_id| MeritId::DemenseNoManse(*unique_id))
            .for_each(|merit_id| output.push(merit_id));

        // Hearthstones and manses
        self.0
            .hearthstones()
            .iter()
            .filter_map(|hearthstone_id| self.0.hearthstones().get(hearthstone_id))
            .for_each(|hearthstone| {
                if hearthstone.manse_and_demense().is_some() {
                    let hearthstone_id = hearthstone.id();
                    output.push(MeritId::Manse(hearthstone_id));
                    output.push(MeritId::DemenseWithManse(hearthstone_id));
                    output.push(MeritId::HearthstoneWithManse(hearthstone_id));
                } else {
                    output.push(MeritId::HearthstoneNoManse(hearthstone.id()));
                }
            });

        // Exalted healing
        match &self.0.exaltation {
            Exaltation::Mortal(mortal) => {
                if mortal.exalted_healing {
                    output.push(MeritId::ExaltedHealing);
                }
            }
            Exaltation::Exalt(_) => output.push(MeritId::ExaltedHealing),
        }

        // Non-native languages
        let mut local_added = false;
        self.0.languages().iter().for_each(|(language, is_native)| {
            if !is_native {
                match language {
                    Language::MajorLanguage(major) => output.push(MeritId::MajorLanguage(major)),
                    Language::LocalTongue(_) => {
                        if !local_added {
                            output.push(MeritId::LocalTongues);
                            local_added = true;
                        }
                    }
                }
            }
        });

        // Martial arts
        self.0
            .martial_arts()
            .iter()
            .map(MeritId::MartialArtist)
            .for_each(|merit_id| output.push(merit_id));

        // Mortal sorcerer
        match &self.0.exaltation {
            Exaltation::Mortal(mortal) => {
                if mortal.sorcery.is_some() {
                    output.push(MeritId::MortalSorcerer);
                }
            }
            Exaltation::Exalt(_) => {}
        }

        // Non-stackable merits
        self.0
            .nonstackable_merits
            .keys()
            .map(|nonstackable_merit_id| MeritId::NonStackable(*nonstackable_merit_id))
            .for_each(|merit_id| output.push(merit_id));

        // Stackable merits
        self.0
            .stackable_merits
            .keys()
            .map(|stackable_merit_id| MeritId::Stackable(*stackable_merit_id))
            .for_each(|merit_id| output.push(merit_id));

        // Sorcery merits
        if let Some(sorcery) = self.0.sorcery() {
            sorcery
                .archetypes()
                .filter_map(|archetype_id| sorcery.archetype(archetype_id))
                .for_each(|(_, merits)| {
                    merits.keys().for_each(|sorcery_archetype_merit_id| {
                        output.push(MeritId::SorceryArchetype(*sorcery_archetype_merit_id));
                    })
                })
        }

        output.into_iter()
    }
}
