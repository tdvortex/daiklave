/// Details for an individual merit.
pub mod merit;

pub mod merit_new;

mod merits_new;

use crate::{
    armor::armor_item::ArmorName, artifact::ArtifactName, exaltation::Exaltation,
    languages::language::{Language, LanguageMutation}, weapons::weapon::WeaponName, Character,
};

use self::merit::{Merit, MeritInstanceName, MeritSource};

/// The merits possessed by a character.
pub struct Merits<'view, 'source>(pub(crate) &'view Character<'source>);

impl<'view, 'source> Merits<'view, 'source> {
    /// Gets a specific Merit belonging to the character (if it exists).
    pub fn get(&self, merit_id: MeritInstanceName<'_>) -> Option<Merit<'source>> {
        match merit_id {
            MeritInstanceName::Artifact(artifact_id) => match artifact_id {
                ArtifactName::Weapon(search_name) => self
                    .0
                    .weapons()
                    .iter()
                    .find_map(|(source_name, equipped)| {
                        if let WeaponName::Artifact(source_artifact_name) = source_name {
                            if source_artifact_name == search_name {
                                self.0
                                    .weapons()
                                    .get(source_name, equipped)
                                    .map(|weapon| (source_artifact_name, weapon))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .and_then(|(name, weapon)| {
                        weapon.merit_dots().map(|dots| {
                            Merit(MeritSource::Artifact(ArtifactName::Weapon(name), dots))
                        })
                    }),
                ArtifactName::Armor(name) => self
                    .0
                    .armor()
                    .get(ArmorName::Artifact(name))
                    .and_then(|armor| {
                        if let ArmorName::Artifact(name) = armor.name() {
                            armor.merit_dots().map(|dots| {
                                Merit(MeritSource::Artifact(ArtifactName::Armor(name), dots))
                            })
                        } else {
                            None
                        }
                    }),
                ArtifactName::Wonder(wonder_name) => {
                    self.0.wonders().get(wonder_name).map(|wonder| {
                        Merit(MeritSource::Artifact(
                            ArtifactName::Wonder(wonder.name()),
                            wonder.merit_dots(),
                        ))
                    })
                }
            },
            MeritInstanceName::Demense(demense_name) => self
                .0
                .demenses_no_manse
                .get_key_value(demense_name)
                .map(|(name, geomancy)| Merit(MeritSource::DemenseNoManse(*name, *geomancy)))
                .or_else(|| {
                    self.0.hearthstones().iter().find_map(|hearthstone_name| {
                        self.0
                            .hearthstones()
                            .get(hearthstone_name)
                            .and_then(|hearthstone| {
                                hearthstone.manse_and_demense().and_then(
                                    |(_, actual_demense_name)| {
                                        if actual_demense_name == demense_name {
                                            Some(Merit(MeritSource::DemenseWithManse(
                                                hearthstone.name(),
                                                actual_demense_name,
                                                hearthstone.geomancy_level(),
                                            )))
                                        } else {
                                            None
                                        }
                                    },
                                )
                            })
                    })
                }),
            MeritInstanceName::ExaltedHealing => match &self.0.exaltation {
                Exaltation::Mortal(mortal) => {
                    if mortal.exalted_healing {
                        Some(Merit(MeritSource::ExaltedHealing(false)))
                    } else {
                        None
                    }
                }
                Exaltation::Exalt(_) => Some(Merit(MeritSource::ExaltedHealing(true))),
            },
            MeritInstanceName::Hearthstone(hearthstone_name) => self
                .0
                .hearthstones()
                .get(hearthstone_name)
                .map(|hearthstone| if hearthstone.manse_and_demense().is_some() {
                    Merit(MeritSource::HearthstoneWithManse(hearthstone.name(), hearthstone.geomancy_level()))
                } else {
                    Merit(MeritSource::HearthstoneNoManse(hearthstone.name(), hearthstone.geomancy_level()))
                }),
            MeritInstanceName::Manse(hearthstone_name) => self
                .0
                .hearthstones()
                .get(hearthstone_name)
                .and_then(|hearthstone| {
                    hearthstone.manse_and_demense().map(|(manse, _)| {
                        Merit(MeritSource::Manse(
                            hearthstone.name(),
                            manse,
                            hearthstone.geomancy_level(),
                        ))
                    })
                }),
            MeritInstanceName::MartialArtist(style_name) => match &self.0.exaltation {
                Exaltation::Mortal(mortal) => mortal
                    .martial_arts_styles
                    .get_key_value(style_name)
                    .map(|(k, _)| Merit(MeritSource::MartialArtist(*k))),
                Exaltation::Exalt(exalt) => exalt
                    .martial_arts_styles
                    .get_key_value(style_name)
                    .map(|(k, _)| Merit(MeritSource::MartialArtist(*k))),
            },
            MeritInstanceName::MortalSorcerer => {
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
            MeritInstanceName::NonStackable(nonstackable_id) => self
                .0
                .nonstackable_merits
                .get(&nonstackable_id)
                .map(|merit| Merit(MeritSource::NonStackable(nonstackable_id, merit.clone()))),
            MeritInstanceName::Stackable(template_name, instance_name) => self
                .0
                .stackable_merits
                .get_key_value(&(template_name, instance_name))
                .map(|((template_name, detail), merit)| Merit(MeritSource::Stackable(*template_name, *detail, *merit))),
            MeritInstanceName::LocalTongues => {
                let purchased = self
                    .0
                    .other_languages
                    .iter()
                    .filter(|&language| {
                        matches!(language, LanguageMutation::LocalTongue(_))
                    })
                    .count();

                if purchased > 0 {
                    Some(Merit(MeritSource::LocalTongues(purchased)))
                } else {
                    None
                }
            }
            MeritInstanceName::MajorLanguage(major) => {
                if self
                    .0
                    .other_languages
                    .contains(&LanguageMutation::MajorLanguage(major))
                {
                    Some(Merit(MeritSource::MajorLanguage(major)))
                } else {
                    None
                }
            }
            MeritInstanceName::SorceryArchetype(archetype_name, merit_name) => {
                self
                .0
                .sorcery()
                .and_then(|sorcery|sorcery.archetype(archetype_name))
                .and_then(|archetype| archetype.merits().get(merit_name))
                .map(|sorcery_archetype_merit| Merit(MeritSource::SorceryArchetype(sorcery_archetype_merit)))
            }
        }
    }

    /// Iterates over all Merits owned by the character by their Id.
    pub fn iter(&self) -> impl Iterator<Item = MeritInstanceName> + '_ {
        // Collect merits Ids into a single vec to minimize heap allocations
        let mut output: Vec<MeritInstanceName> = Vec::new();

        // Artifact weapons
        self.0
            .weapons()
            .iter()
            .filter_map(|(name, equipped)| {
                self.0.weapons().get(name, equipped).and_then(|weapon| {
                    if let WeaponName::Artifact(artifact_weapon_name) = weapon.name() {
                        Some(MeritInstanceName::Artifact(ArtifactName::Weapon(
                            artifact_weapon_name,
                        )))
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
            .filter_map(|name| {
                if let ArmorName::Artifact(name) = name {
                    Some(MeritInstanceName::Artifact(ArtifactName::Armor(name)))
                } else {
                    None
                }
            })
            .for_each(|merit_id| output.push(merit_id));

        // Wonders
        self.0
            .wonders()
            .iter()
            .map(|name| MeritInstanceName::Artifact(ArtifactName::Wonder(name)))
            .for_each(|merit_id| output.push(merit_id));

        // Demenses without manses
        self.0
            .demenses_no_manse
            .keys()
            .map(|unique_id| MeritInstanceName::Demense(*unique_id))
            .for_each(|merit_id| output.push(merit_id));

        // Hearthstones and manses
        self.0
            .hearthstones()
            .iter()
            .filter_map(|hearthstone_id| self.0.hearthstones().get(hearthstone_id))
            .for_each(|hearthstone| {
                let hearthstone_name = hearthstone.name();
                output.push(MeritInstanceName::Hearthstone(hearthstone_name));
                if let Some((manse, demense)) = hearthstone.manse_and_demense() {
                    output.push(MeritInstanceName::Manse(manse));
                    output.push(MeritInstanceName::Demense(demense));
                }
            });

        // Exalted healing
        match &self.0.exaltation {
            Exaltation::Mortal(mortal) => {
                if mortal.exalted_healing {
                    output.push(MeritInstanceName::ExaltedHealing);
                }
            }
            Exaltation::Exalt(_) => output.push(MeritInstanceName::ExaltedHealing),
        }

        // Non-native languages
        let mut local_added = false;
        self.0.languages().iter().for_each(|(language, is_native)| {
            if !is_native {
                match language {
                    Language::MajorLanguage(major) => {
                        output.push(MeritInstanceName::MajorLanguage(major))
                    }
                    Language::LocalTongue(_) => {
                        if !local_added {
                            output.push(MeritInstanceName::LocalTongues);
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
            .map(MeritInstanceName::MartialArtist)
            .for_each(|merit_id| output.push(merit_id));

        // Mortal sorcerer
        match &self.0.exaltation {
            Exaltation::Mortal(mortal) => {
                if mortal.sorcery.is_some() {
                    output.push(MeritInstanceName::MortalSorcerer);
                }
            }
            Exaltation::Exalt(_) => {}
        }

        // Non-stackable merits
        self.0
            .nonstackable_merits
            .keys()
            .map(|nonstackable_merit_id| MeritInstanceName::NonStackable(*nonstackable_merit_id))
            .for_each(|merit_id| output.push(merit_id));

        // Stackable merits
        self.0
            .stackable_merits
            .keys()
            .map(|(template_name, detail)| MeritInstanceName::Stackable(*template_name, *detail))
            .for_each(|merit_id| output.push(merit_id));

        // Sorcery merits
        if let Some(sorcery) = self.0.sorcery() {
            sorcery
                .archetypes()
                .filter_map(|name| sorcery.archetype(name))
                .flat_map(|archetype| archetype
                    .merits()
                    .iter()
                    .map(|merit_name| MeritInstanceName::SorceryArchetype(archetype.name(), merit_name))
                    .collect::<Vec<MeritInstanceName>>()
                    .into_iter()
                )
                .for_each(|merit_id| output.push(merit_id));
        }

        output.into_iter()
    }
}
