use std::collections::hash_map::Entry;

use crate::{
    abilities::{AbilityName, AbilityNameVanilla},
    merits::{
        merit::{
            MeritError, MeritPrerequisite, NonStackableMerit, NonStackableMeritId, StackableMerit,
            StackableMeritId,
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
        stackable_merit_id: StackableMeritId,
        stackable_merit: &'source StackableMerit,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.validate_merit_prerequisites(stackable_merit.prerequisites())?;
        if let Entry::Vacant(e) = self.stackable_merits.entry(stackable_merit_id) {
            e.insert(stackable_merit.as_ref());
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
        stackable_merit_id: StackableMeritId,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.stackable_merits.remove(&stackable_merit_id).is_some() {
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
                                if self.abilities().get(vanilla).dots() >= dots_required.get() {
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
        nonstackable_merit_id: NonStackableMeritId,
        nonstackable_merit: &'source NonStackableMerit,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.validate_merit_prerequisites(nonstackable_merit.prerequisites())?;

        if let Entry::Vacant(e) = self.nonstackable_merits.entry(nonstackable_merit_id) {
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
        nonstackable_merit_id: NonStackableMeritId,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self
            .nonstackable_merits
            .remove(&nonstackable_merit_id)
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
            .filter_map(|(id, merit)| {
                if self
                    .validate_merit_prerequisites(merit.prerequisites())
                    .is_err()
                {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect::<Vec<NonStackableMeritId>>()
            .into_iter()
            .for_each(|id| {
                self.nonstackable_merits.remove(&id);
            });

        self.stackable_merits
            .iter()
            .filter_map(|(id, merit)| {
                if self
                    .validate_merit_prerequisites(merit.prerequisites())
                    .is_err()
                {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect::<Vec<StackableMeritId>>()
            .into_iter()
            .for_each(|id| {
                self.stackable_merits.remove(&id);
            });
    }
}
