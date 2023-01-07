/// Properties of individual pieces of armor
pub mod armor_item;
mod armor_weight;

pub use armor_weight::ArmorWeightClass;

use crate::exaltation::Exaltation;

use self::armor_item::{ArmorItem, ArmorId};

/// An interface for all of a character's armor. For an individual piece of
/// armor, see ArmorItem.
pub struct Armor<'view, 'source>(pub(crate) &'view Exaltation<'source>);

impl<'view, 'source> Armor<'view, 'source> {
    /// Iterates over all owned pieces of armor by their Id.
    pub fn iter(&self) -> impl Iterator<Item = ArmorId> + '_ {
        vec![].into_iter()
    }

    /// Gets the currently worn piece of armor, if eny.
    pub fn worn(&self) -> Option<(ArmorId, ArmorItem<'source>)> {
        todo!()
    }

    /// Gets a piece of armor by its Id.
    pub fn get(&self, armor_id: ArmorId) -> Option<ArmorItem<'source>> {
        todo!()
    }
}