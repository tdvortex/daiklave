use std::collections::{HashMap, HashSet};

use crate::{abilities::Ability, charms::MartialArtsCharm, id::Id};

use super::{MartialArtistTraits, MartialArtsStyle};

pub struct MartialArtsDiff {
    removed_styles: Vec<Id>,
    added_styles: Vec<(
        MartialArtsStyle,
        u8,
        Option<Vec<String>>,
        Vec<MartialArtsCharm>,
    )>,
    modified_style: Vec<(Id, u8, Option<Vec<String>>, Vec<MartialArtsCharm>)>,
}

impl MartialArtistTraits {
    pub fn compare_newer(&self, newer: &MartialArtistTraits) -> MartialArtsDiff {
        let mut diff = MartialArtsDiff {
            removed_styles: Vec::new(),
            added_styles: Vec::new(),
            modified_style: Vec::new(),
        };
        let mut old_hashmap: HashMap<Id, (&MartialArtsStyle, Ability, &Vec<MartialArtsCharm>)> =
            self.iter()
                .map(|(style_ptr, ability, vec_ptr)| {
                    (style_ptr.id(), (style_ptr, ability, vec_ptr))
                })
                .collect();

        for (style_ptr, ability, vec_ptr) in newer.iter() {
            if !old_hashmap.contains_key(&style_ptr.id()) {
                diff.added_styles.push((
                    style_ptr.clone(),
                    ability.dots(),
                    ability.specialties().map(|v| v.clone()),
                    vec_ptr.clone(),
                ));
            } else {
                let (_, old_ability, old_vec_ptr) = old_hashmap.remove(&style_ptr.id()).unwrap();

                if ability.dots() != old_ability.dots()
                    || ability.specialties() != old_ability.specialties()
                    || vec_ptr != old_vec_ptr
                {
                    diff.modified_style.push((
                        style_ptr.id(),
                        ability.dots(),
                        ability.specialties().map(|v| v.clone()),
                        vec_ptr.clone(),
                    ));
                }
            }
        }

        for id in old_hashmap.keys() {
            diff.removed_styles.push(*id);
        }

        diff
    }
}
