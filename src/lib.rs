pub mod abilities;
pub mod armor;
pub mod attributes;
pub mod campaign;
pub mod character;
pub(crate) mod charms;
pub mod data_source;
pub mod health;
pub mod intimacies;
pub mod merits;
pub mod player;
pub mod prerequisite;
pub mod weapons;
use std::{collections::HashSet, hash::Hash};

pub use character::{retrieve_character, update_character, Character};
pub use player::{create_player, destroy_player};
use slab::Slab;

pub(crate) fn slab_eq<T: PartialEq + Eq + Hash>(self_slab: &Slab<T>, other_slab: &Slab<T>) -> bool {
    if self_slab.len() != other_slab.len() {
        // Slabs must contain the same number of items
        return false;
    }

    if self_slab.is_empty() && other_slab.is_empty() {
        // Two empty slabs are the same
        return true;
    }

    const SHORT_SLAB_LEN: usize = 16;

    if self_slab.len() <= SHORT_SLAB_LEN {
        // For short slabs, avoid heap allocations
        let mut self_refs: [Option<&T>; SHORT_SLAB_LEN] = [None; SHORT_SLAB_LEN];
        let mut occupied = 0;
        for self_ref in self_slab.iter().map(|(_, t_ref)| t_ref) {
            self_refs[occupied] = Some(self_ref);
            occupied += 1;
        }

        for other_ref in other_slab.iter().map(|(_, t_ref)| t_ref) {
            let mut found_index = None;
            for (index, self_ref_option) in self_refs.iter().take(occupied).enumerate() {
                if *self_ref_option == Some(other_ref) {
                    found_index = Some(index);
                    break;
                }
            }

            if let Some(index) = found_index {
                self_refs[index] = None;
                std::mem::swap(&mut self_refs[index], &mut self_refs[occupied]);
                occupied -= 1;
            } else {
                return false;
            }
        }
        true
    } else {
        // For larger slabs, just use a hashset
        let mut self_refs: HashSet<&T> = self_slab.iter().map(|(_, t_ref)| t_ref).collect();
        for other_ref in other_slab.iter().map(|(_, t_ref)| t_ref) {
            if !self_refs.remove(other_ref) {
                return false;
            }
        }
        true
    }
}
