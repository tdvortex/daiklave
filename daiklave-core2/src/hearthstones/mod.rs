mod error;

/// Structs related to an individual hearthstone.
pub mod hearthstone;

use crate::Character;

pub use error::HearthstoneError;
pub(crate) use hearthstone::{
    HearthstoneOrigin, HearthstonePosition, HearthstoneStability, SlottedHearthstone,
    SlottedHearthstoneMemo, UnslottedHearthstone, UnslottedHearthstoneMemo,
};

use self::hearthstone::Hearthstone;

/// The Hearthstones owned by a character, their current position, and any
/// Manses and Demenses they may also have.
pub struct Hearthstones<'view, 'source>(pub(crate) &'view Character<'source>);

impl<'view, 'source> Hearthstones<'view, 'source> {
    fn get_unslotted(&self, name: &str) -> Option<Hearthstone<'source>> {
        self.0
            .hearthstone_inventory
            .get_key_value(name)
            .map(|(name, unslotted)| Hearthstone(HearthstonePosition::Unslotted(*name, *unslotted)))
    }

    fn get_weapon_slotted(&self, name: &str) -> Option<Hearthstone<'source>> {
        self.0.weapons().iter().find_map(|(weapon_id, equipped)| {
            self.0
                .weapons()
                .get(weapon_id, equipped)
                .and_then(|weapon| {
                    weapon
                        .slotted_hearthstones()
                        .find(|hearthstone| hearthstone.name() == name)
                })
        })
    }

    fn get_armor_slotted(&self, name: &str) -> Option<Hearthstone<'source>> {
        self.0.armor().iter().find_map(|armor_id| {
            self.0.armor().get(armor_id).and_then(|armor_item| {
                armor_item
                    .slotted_hearthstones()
                    .find(|hearthstone| hearthstone.name() == name)
            })
        })
    }

    fn get_wonder_slotted(&self, name: &str) -> Option<Hearthstone<'source>> {
        self.0.wonders().iter().find_map(|wonder_id| {
            self.0.wonders().get(wonder_id).and_then(|owned_wonder| {
                owned_wonder
                    .slotted_hearthstones()
                    .find(|hearthstone| hearthstone.name() == name)
            })
        })
    }

    /// Gets the details of a specific hearthstone by its Id.
    pub fn get(&self, name: &str) -> Option<Hearthstone<'source>> {
        self.get_unslotted(name)
            .or_else(|| self.get_weapon_slotted(name))
            .or_else(|| self.get_armor_slotted(name))
            .or_else(|| self.get_wonder_slotted(name))
    }

    /// Iterates over all hearthstones owned by the character by their names.
    pub fn iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        self.0
            .hearthstone_inventory
            .keys()
            .copied()
            .chain(self.0.weapons().iter().flat_map(|(weapon_id, equipped)| {
                self.0
                    .weapons()
                    .get(weapon_id, equipped)
                    .iter()
                    .flat_map(|weapon| {
                        weapon
                            .slotted_hearthstones()
                            .map(|hearthstone| hearthstone.name())
                    })
                    .collect::<Vec<&str>>()
                    .into_iter()
            }))
            .chain(self.0.armor().iter().flat_map(|armor_id| {
                self.0
                    .armor()
                    .get(armor_id)
                    .iter()
                    .flat_map(|armor| {
                        armor
                            .slotted_hearthstones()
                            .map(|hearthstone| hearthstone.name())
                    })
                    .collect::<Vec<&str>>()
                    .into_iter()
            }))
            .chain(self.0.wonders().iter().flat_map(|wonder_id| {
                self.0
                    .wonders()
                    .get(wonder_id)
                    .iter()
                    .flat_map(|armor| {
                        armor
                            .slotted_hearthstones()
                            .map(|hearthstone| hearthstone.name())
                    })
                    .collect::<Vec<&str>>()
                    .into_iter()
            }))
            .collect::<Vec<&str>>()
            .into_iter()
    }
}
