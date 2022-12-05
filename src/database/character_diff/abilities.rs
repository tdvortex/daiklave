use crate::character::traits::abilities::{Abilities, AbilityName};

#[derive(Debug, Default)]
struct AbilitiesDiff {
    upserted_abilities: Vec<(AbilityName, u8)>,
    removed_abilities: Vec<AbilityName>,
    added_specialties: Vec<(AbilityName, String)>,
    removed_specialties: Vec<(AbilityName, String)>,
}

impl Abilities {
    fn compare_newer(&self, newer: &Abilities) -> AbilitiesDiff {
        let mut diff = AbilitiesDiff::default();

        for old_ability in self.iter() {
            match (newer.get(old_ability.name())) {
                Some(new_ability) => {
                    if old_ability.dots() != new_ability.dots() {
                        diff.upserted_abilities.push((old_ability.name().clone(), new_ability.dots()));
                    }

                    match (old_ability.specialties(), new_ability.specialties()) {
                        (None, None) => {}
                        (None, Some(specialties)) => {
                            diff.added_specialties.extend(
                                specialties
                                    .iter()
                                    .map(|string_ref| (old_ability.name().clone(), string_ref.clone())),
                            );
                        }
                        (Some(specialties), None) => {
                            diff.removed_specialties.extend(
                                (*specialties)
                                    .iter()
                                    .map(|string_ref| (old_ability.name().clone(), string_ref.clone())),
                            );
                        }
                        (Some(old), Some(new)) => {
                            diff.removed_specialties.extend(
                                old.difference(new).map(|specialty| (old_ability.name().clone(), specialty.clone())));
    
                            diff.added_specialties.extend(
                                new.difference(old).map(|specialty| (old_ability.name().clone(), specialty.clone())));
                        }
                    }
                }
                None => {
                    diff.removed_abilities.push(old_ability.name().clone());
                }
            }
        }

        for new_ability in newer.iter() {
            if self.get(new_ability.name()).is_none() {
                diff.upserted_abilities.push((new_ability.name().clone(), new_ability.dots()));
                if new_ability.specialties().is_some() {
                    diff.added_specialties.extend(
                        new_ability
                            .specialties()
                            .unwrap()
                            .iter()
                            .map(|string_ref| (new_ability.name().clone(), string_ref.clone()))
                    );
                }
            }
        }

        diff
    }

    fn compare_older(&self, older: &Abilities) -> AbilitiesDiff {
        older.compare_newer(self)
    }
}
