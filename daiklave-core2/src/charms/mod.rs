/// Traits for individual Charms.
pub mod charm;

mod action_type;
mod cost;
mod cost_type;
mod keyword;

pub use action_type::CharmActionType;
pub use cost::CharmCost;
pub use cost_type::CharmCostType;
pub use keyword::CharmKeyword;

use crate::Character;

use self::charm::{CharmId, Charm};

/// The interface for all the Charms a character possesses.
pub struct Charms<'view, 'source>(pub(crate) &'view Character<'source>);

impl<'view, 'source> Charms<'view, 'source> {
    /// Iterates over all Charms (including Spells and Evocations) owned by the
    /// character by their Ids.
    pub fn iter(&self) -> impl Iterator<Item = CharmId> + '_ {
        vec![].into_iter()
    }

    /// Retrieves a specific Charm by its Id, or returns None if not found.
    pub fn get(&self, charm_id: CharmId) -> Option<Charm<'source>> {
        todo!()
    }
}