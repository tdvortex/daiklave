use std::collections::hash_map::Entry;

use crate::{
    abilities::{AbilityName, AbilityNameVanilla},
    merits::{
        merit_new::{
            MeritError, MeritPrerequisite, NonStackableMerit, StackableMerit, Merit, MeritSource, AddStackableMerit, AddNonStackableMerit, AddMerit, RemoveMerit,
        },
    },
    Character, CharacterMutationError, exaltation::Exaltation, languages::language::LanguageMutation,
};

impl<'view, 'source> Character<'source> {
    /// Access all Merits owned by the character.
    pub fn merits(&self) -> Vec<Merit<'source>> {
        let armor = self.armor();
        let from_armor = armor
            .iter()
            .filter_map(|armor_name| armor.get(armor_name))
            .flat_map(|armor_item| armor_item.merits().into_iter());

        let weapons = self.weapons();
        let from_weapons = weapons
            .iter()
            .filter_map(|(name, equipped)| weapons.get(name, equipped))
            .flat_map(|weapon| weapon.merits().into_iter());

        let wonders = self.wonders();
        let from_wonders = wonders
            .iter()
            .filter_map(|name| wonders.get(name))
            .flat_map(|wonder| wonder.merits().into_iter());

        let demenses = self.demenses_no_manse.iter().map(|(name, level)| {
            Merit(MeritSource::Demense {
                name: *name,
                has_manse: false,
                geomancy_level: *level,
            })
        });

        let hearthstones = self.hearthstones();
        let from_hearthstones = hearthstones
            .iter()
            .filter_map(|name| hearthstones.get(name))
            .flat_map(|hearthstone| hearthstone.merits().into_iter());

        let exalted_healing = if let Exaltation::Mortal(mortal) = &self.exaltation {
            if mortal.exalted_healing {
                vec![Merit(MeritSource::ExaltedHealing { is_exalt: false })]
            } else {
                vec![]
            }
        } else {
            vec![Merit(MeritSource::ExaltedHealing { is_exalt: true })]
        }
        .into_iter();

        let (local_count, mut language_merits) = self.other_languages.iter().fold(
            (0, Vec::new()),
            |(local_count, mut majors), language| match language {
                LanguageMutation::MajorLanguage(major) => {
                    majors.push(Merit(MeritSource::MajorLanguage(*major)));
                    (local_count, majors)
                }
                LanguageMutation::LocalTongue(_) => (local_count + 1, majors),
            },
        );
        if local_count > 0 {
            language_merits.push(Merit(MeritSource::LocalTongues { count: local_count }));
        }
        let languages = language_merits.into_iter();

        let martial_artists = self
            .martial_arts()
            .iter()
            .map(|style_name| Merit(MeritSource::MartialArtist { style_name }));

        let mortal_sorcerer = if let Exaltation::Mortal(mortal) = &self.exaltation {
            if mortal.sorcery.is_some() {
                vec![Merit(MeritSource::MortalSorcerer)]
            } else {
                vec![]
            }
        } else {
            vec![]
        }
        .into_iter();

        let nonstackable = self.nonstackable_merits.iter().map(|(name, instance)| {
            Merit(MeritSource::NonStackable(NonStackableMerit {
                name,
                instance,
            }))
        });
        let stackable =
            self
                .stackable_merits
                .iter()
                .map(|((template_name, detail), instance)| {
                    Merit(MeritSource::Stackable(StackableMerit {
                        template_name,
                        detail,
                        instance,
                    }))
                });
        let maybe_sorcery = self.sorcery();
        let sorcery_merits = maybe_sorcery.iter().flat_map(|sorcery| {
            sorcery
                .archetypes()
                .filter_map(|name| sorcery.archetype(name))
                .flat_map(|with_merits| with_merits.merits().iter().collect::<Vec<Merit>>().into_iter())
        });

        from_armor
            .chain(from_weapons)
            .chain(from_wonders)
            .chain(demenses)
            .chain(from_hearthstones)
            .chain(exalted_healing)
            .chain(languages)
            .chain(martial_artists)
            .chain(mortal_sorcerer)
            .chain(nonstackable)
            .chain(stackable)
            .chain(sorcery_merits)
            .collect()
    }

    pub fn add_merit(&mut self, add_merit: &'source AddMerit) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }

    pub fn remove_merit(&mut self, remove_merit: &RemoveMerit) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }

    /// Adds a stackable merit to the character.
    pub fn add_stackable_merit(
        &mut self,
        add_stackable_merit: &'source AddStackableMerit,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.validate_merit_prerequisites(add_stackable_merit.instance.0.prerequisites.iter().copied())?;
        if let Entry::Vacant(e) = self.stackable_merits.entry((&add_stackable_merit.template_name, &add_stackable_merit.detail)) {
            e.insert(&add_stackable_merit.instance);
            Ok(self)
        } else {
            Err(CharacterMutationError::MeritError(
                MeritError::DuplicateMerit,
            ))
        }
    }

    /// Removes a nonstackable merit from the character.
    pub fn remove_stackable_merit(
        &mut self,
        template_name: &'source str,
        detail: &'source str,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.stackable_merits.remove(&(template_name, detail)).is_some() {
            Ok(self)
        } else {
            Err(CharacterMutationError::MeritError(MeritError::NotFound))
        }
    }

    fn validate_merit_prerequisites<P>(
        &self,
        prerequisites: P,
    ) -> Result<(), CharacterMutationError>
    where
        P: ExactSizeIterator<Item = MeritPrerequisite>,
    {
        if prerequisites.len() > 0 {
            let mut qualified = false;
            for prereq in prerequisites {
                match prereq {
                    MeritPrerequisite::Ability(ability_name, dots_required) => match ability_name {
                        AbilityName::Craft => {
                            if self.craft().max() >= dots_required.get() {
                                qualified = true;
                                break;
                            }
                        }
                        AbilityName::MartialArts => {
                            if self
                                .martial_arts()
                                .iter()
                                .map(|style_id| {
                                    self.martial_arts()
                                        .style(style_id)
                                        .map_or(0, |martial_artist| martial_artist.ability().dots())
                                })
                                .max()
                                .unwrap_or(0)
                                >= dots_required.get()
                            {
                                qualified = true;
                                break;
                            }
                        }
                        other_ability => {
                            if let Ok(vanilla) = AbilityNameVanilla::try_from(other_ability) {
                                if self.abilities().get_vanilla(vanilla).dots() >= dots_required.get() {
                                    qualified = true;
                                    break;
                                }
                            }
                        }
                    },
                    MeritPrerequisite::Attribute(attribute_name, dots_required) => {
                        if self.attributes().dots(attribute_name) >= dots_required {
                            qualified = true;
                            break;
                        }
                    }
                }
            }
            if !qualified {
                return Err(CharacterMutationError::MeritError(
                    MeritError::PrerequisitesNotMet,
                ));
            }
        }
        Ok(())
    }

    /// Adds a nonstackable merit to the character.
    pub fn add_nonstackable_merit(
        &mut self,
        add_nonstackable_merit: &'source AddNonStackableMerit,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.validate_merit_prerequisites(add_nonstackable_merit.instance.0.prerequisites.iter().copied())?;

        if let Entry::Vacant(e) = self.nonstackable_merits.entry(&add_nonstackable_merit.name) {
            e.insert(&add_nonstackable_merit.instance);
            Ok(self)
        } else {
            Err(CharacterMutationError::MeritError(
                MeritError::DuplicateMerit,
            ))
        }
    }

    /// Removes a nonstackable merit from the character.
    pub fn remove_nonstackable_merit(
        &mut self,
        nonstackable_merit_name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self
            .nonstackable_merits
            .remove(nonstackable_merit_name)
            .is_some()
        {
            Ok(self)
        } else {
            Err(CharacterMutationError::MeritError(MeritError::NotFound))
        }
    }

    /// Adds the Exalted Healing merit to the character.
    pub fn add_exalted_healing(&mut self) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.exaltation {
            crate::exaltation::Exaltation::Mortal(mortal) => {
                if mortal.exalted_healing {
                    Err(CharacterMutationError::MeritError(
                        MeritError::DuplicateMerit,
                    ))
                } else {
                    mortal.exalted_healing = true;
                    Ok(self)
                }
            }
            crate::exaltation::Exaltation::Exalt(_) => Err(CharacterMutationError::MeritError(
                MeritError::DuplicateMerit,
            )),
        }
    }

    /// Removes the Exalted Healing merit from the character.
    pub fn remove_exalted_healing(&mut self) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.exaltation {
            crate::exaltation::Exaltation::Mortal(mortal) => {
                if !mortal.exalted_healing {
                    Err(CharacterMutationError::MeritError(MeritError::NotFound))
                } else {
                    mortal.exalted_healing = false;
                    Ok(self)
                }
            }
            crate::exaltation::Exaltation::Exalt(_) => Err(CharacterMutationError::MeritError(
                MeritError::RemoveExaltedHealing,
            )),
        }
    }

    pub(crate) fn correct_merits(&mut self) {
        self.nonstackable_merits
            .iter()
            .filter_map(|(name, merit)| {
                if self
                    .validate_merit_prerequisites(merit.0.prerequisites.iter().copied())
                    .is_err()
                {
                    Some(*name)
                } else {
                    None
                }
            })
            .collect::<Vec<&str>>()
            .into_iter()
            .for_each(|id| {
                self.nonstackable_merits.remove(&id);
            });

        self.stackable_merits
            .iter()
            .filter_map(|((template_name, detail), merit)| {
                if self
                    .validate_merit_prerequisites(merit.0.prerequisites.iter().copied())
                    .is_err()
                {
                    Some((*template_name, *detail))
                } else {
                    None
                }
            })
            .collect::<Vec<(&str, &str)>>()
            .into_iter()
            .for_each(|key| {
                self.stackable_merits.remove(&key);
            });
    }
}
