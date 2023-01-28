/// Traits for individual Charms.
pub mod charm;

mod action_type;
mod cost;
mod cost_type;
mod error;
mod keyword;

pub use action_type::CharmActionType;
pub use cost::CharmCost;
pub use cost_type::CharmCostType;
pub use error::CharmError;
pub use keyword::CharmKeyword;

use crate::{exaltation::Exaltation, Character};

use self::charm::{Charm, CharmName};

/// The interface for all the Charms a character possesses.
pub struct Charms<'view, 'source>(pub(crate) &'view Character<'source>);

impl<'view, 'source> Charms<'view, 'source> {
    /// Iterates over all Charms (including Spells and Evocations) owned by the
    /// character by their Ids.
    pub fn iter(&self) -> impl Iterator<Item = CharmName> + '_ {
        let solar_charms = self.0.solar_charms_iter().map(CharmName::Solar);

        let eclipse_charms = self
            .0
            .exaltation
            .eclipse_charms_iter()
            .map(CharmName::Spirit);

        let evocations = if let Exaltation::Exalt(exalt) = &self.0.exaltation {
            exalt
                .evocations
                .iter()
                .map(|(evocation_id, _)| CharmName::Evocation(*evocation_id))
                .collect::<Vec<CharmName>>()
        } else {
            vec![]
        }
        .into_iter();

        let martial_arts_charms = self
            .0
            .exaltation
            .martial_arts_charms_iter()
            .map(CharmName::MartialArts);

        let spells = if let Some(sorcery) = self.0.sorcery() {
            sorcery
                .spells()
                .iter()
                .map(CharmName::Spell)
                .collect::<Vec<CharmName>>()
        } else {
            vec![]
        }
        .into_iter();

        solar_charms
            .chain(eclipse_charms)
            .chain(martial_arts_charms)
            .chain(spells)
            .chain(evocations)
    }

    /// Retrieves a specific Charm by its Id, or returns None if not found.
    pub fn get(&self, charm_id: CharmName) -> Option<Charm<'source>> {
        match charm_id {
            CharmName::Spirit(spirit_charm_id) => {
                self.0.exaltation.get_eclipse_charm(spirit_charm_id)
            }
            CharmName::Evocation(evocation_id) => {
                if let Exaltation::Exalt(exalt) = &self.0.exaltation {
                    exalt.evocations.iter().find_map(|(known_id, evocation)| {
                        if known_id == &evocation_id {
                            Some(Charm::Evocation(evocation))
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            }
            CharmName::MartialArts(martial_arts_charm_id) => self
                .0
                .exaltation
                .get_martial_arts_charm(martial_arts_charm_id),
            CharmName::Solar(solar_charm_id) => self.0.exaltation.get_solar_charm(solar_charm_id),
            CharmName::Spell(spell_id) => self
                .0
                .sorcery()
                .and_then(|sorcery| sorcery.spells().get(spell_id))
                .map(|(spell, _)| Charm::Spell(spell)),
        }
    }
}
