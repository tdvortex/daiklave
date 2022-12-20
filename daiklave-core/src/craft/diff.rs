use std::collections::HashSet;

use super::CraftAbilities;

#[derive(Debug, Default)]
pub struct CraftDiff {
    pub upserted_foci: Vec<(String, u8)>,
    pub removed_foci: Vec<String>,
    pub added_specialties: Vec<(String, String)>,
    pub removed_specialties: Vec<(String, String)>,
}

impl CraftAbilities {
    pub(crate) fn compare_newer(&self, newer: &Self) -> CraftDiff {
        let mut diff = CraftDiff::default();

        for ability in newer.iter() {
            if let Some(old_rating) = self.get_rating(ability.name().subskill().unwrap()) {
                if old_rating.dots() != ability.dots() {
                    diff.upserted_foci.push((
                        ability.name().subskill().unwrap().to_owned(),
                        ability.dots(),
                    ));
                }

                match (old_rating.specialties(), ability.specialties()) {
                    (None, None) => {}
                    (None, Some(added)) => {
                        for specialty in added.iter() {
                            diff.added_specialties.push((
                                ability.name().subskill().unwrap().to_owned(),
                                specialty.clone(),
                            ))
                        }
                    }
                    (Some(removed), None) => {
                        for specialty in removed.iter() {
                            diff.removed_specialties.push((
                                ability.name().subskill().unwrap().to_owned(),
                                specialty.clone(),
                            ))
                        }
                    }
                    (Some(old_specialties), Some(new_specialties)) => {
                        let mut old_specialties_set: HashSet<&str> =
                            old_specialties.iter().map(|s| s.as_str()).collect();

                        for specialty in new_specialties {
                            if !old_specialties_set.remove(specialty.as_str()) {
                                diff.added_specialties.push((
                                    ability.name().subskill().unwrap().to_owned(),
                                    specialty.clone(),
                                ))
                            }
                        }

                        for specialty in old_specialties_set.into_iter() {
                            diff.removed_specialties.push((
                                ability.name().subskill().unwrap().to_owned(),
                                specialty.to_owned(),
                            ))
                        }
                    }
                }
            } else {
                diff.upserted_foci.push((
                    ability.name().subskill().unwrap().to_owned(),
                    ability.dots(),
                ));

                if let Some(specialties) = ability.specialties() {
                    for specialty in specialties {
                        diff.added_specialties.push((
                            ability.name().subskill().unwrap().to_owned(),
                            specialty.clone(),
                        ));
                    }
                }
            }
        }

        for ability in self.iter() {
            if newer
                .get_rating(ability.name().subskill().unwrap())
                .is_none()
            {
                diff.removed_foci
                    .push(ability.name().subskill().unwrap().to_owned());
            }
        }

        diff
    }
}
