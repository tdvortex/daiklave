/// Properties of individual pieces of armor
pub mod armor_item;

mod error;
pub use error::ArmorError;

use crate::exaltation::Exaltation;

use self::armor_item::{ArmorItem, ArmorName};

/// An interface for all of a character's armor. For an individual piece of
/// armor, see ArmorItem.
pub struct Armor<'view, 'source>(pub(crate) &'view Exaltation<'source>);

impl<'view, 'source> Armor<'view, 'source> {
    /// Iterates over all owned pieces of armor by their Id.
    pub fn iter(&self) -> impl Iterator<Item = ArmorName<'source>> + '_ {
        self.0.armor_iter()
    }

    /// Gets the currently worn piece of armor, if eny.
    pub fn worn(&self) -> Option<ArmorItem<'source>> {
        self.0.worn_armor()
    }

    /// Gets a piece of armor by its name.
    pub fn get(&self, name: ArmorName<'_>) -> Option<ArmorItem<'source>> {
        self.0.get_armor(name)
    }
}
