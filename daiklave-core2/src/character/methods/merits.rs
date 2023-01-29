use std::collections::hash_map::Entry;

use crate::{
    abilities::{AbilityName, AbilityNameVanilla},
    merits::{
        merit::{
            MeritError, MeritPrerequisite, NonStackableMerit, StackableMerit,
        },
        Merits,
    },
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    /// Access all Merits owned by the character.
    pub fn merits(&'view self) -> Merits<'view, 'source> {
        Merits(self)
    }

    /// Adds a stackable merit to the character.
    pub fn add_stackable_merit(
        &mut self,
        template_name: &'source str,
        instance_name: &'source str,
        merit: &'source StackableMerit,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.validate_merit_prerequisites(merit.prerequisites())?;
        if let Entry::Vacant(e) = self.stackable_merits.entry((template_name, instance_name)) {
            e.insert(merit.into());
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
        template_name: &str,
        instance_name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.stackable_merits.remove(&(template_name, instance_name)).is_some() {
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
        nonstackable_merit_name: &'source str,
        nonstackable_merit: &'source NonStackableMerit,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.validate_merit_prerequisites(nonstackable_merit.prerequisites())?;

        if let Entry::Vacant(e) = self.nonstackable_merits.entry(nonstackable_merit_name) {
            e.insert(nonstackable_merit.as_ref());
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
            .remove(&nonstackable_merit_name)
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
                MeritError::ExaltedHealing,
            )),
        }
    }

    pub(crate) fn correct_merits(&mut self) {
        self.nonstackable_merits
            .iter()
            .filter_map(|(name, merit)| {
                if self
                    .validate_merit_prerequisites(merit.prerequisites())
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
                    .validate_merit_prerequisites(merit.prerequisites())
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
