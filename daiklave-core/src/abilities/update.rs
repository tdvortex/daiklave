use std::collections::HashSet;

use crate::abilities::Abilities;

use super::AbilityNameVanilla;

#[derive(Debug, Default)]
pub struct AbilitiesDiff {
    pub abilities_to_modify: Vec<(AbilityNameVanilla, u8)>,
    pub specialties_to_add: Vec<(AbilityNameVanilla, String)>,
    pub specialties_to_remove: Vec<(AbilityNameVanilla, String)>,
}

impl Abilities {
    pub fn compare_newer(&self, newer: &Self) -> AbilitiesDiff {
        let mut diff = AbilitiesDiff::default();

        for (old_ability, new_ability) in self.iter().zip(newer.iter()) {
            if old_ability.dots() != new_ability.dots() {
                diff.abilities_to_modify.push((
                    old_ability.name().without_subskill().try_into().unwrap(),
                    new_ability.dots(),
                ));
            }

            match (old_ability.specialties(), new_ability.specialties()) {
                (None, None) => {}
                (None, Some(added)) => {
                    for specialty in added.clone().into_iter() {
                        diff.specialties_to_add.push((
                            new_ability.name().without_subskill().try_into().unwrap(),
                            specialty,
                        ));
                    }
                }
                (Some(removed), None) => {
                    for specialty in removed.clone().into_iter() {
                        diff.specialties_to_remove.push((
                            old_ability.name().without_subskill().try_into().unwrap(),
                            specialty,
                        ));
                    }
                }
                (Some(old_specialties), Some(new_specialties)) => {
                    let mut old_set: HashSet<&str> =
                        old_specialties.iter().map(|s| s.as_str()).collect();

                    for specialty in new_specialties.iter() {
                        if !old_set.remove(specialty.as_str()) {
                            diff.specialties_to_add.push((
                                new_ability.name().without_subskill().try_into().unwrap(),
                                specialty.clone(),
                            ));
                        }
                    }

                    for specialty in old_set.into_iter() {
                        diff.specialties_to_remove.push((
                            old_ability.name().without_subskill().try_into().unwrap(),
                            specialty.to_owned(),
                        ));
                    }
                }
            }
        }
        diff
    }
}