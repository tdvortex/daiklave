mod wonder;

pub use wonder::{AddWonder, OwnedWonder, Wonder};
pub(crate) use wonder::{WonderNoAttunement, WonderNoAttunementMemo};

use crate::exaltation::Exaltation;

/// A character's collection of non-armor, non-weapon, non-Warstrider
/// artifacts.
pub struct Wonders<'view, 'source>(pub(crate) &'view Exaltation<'source>);

impl<'view, 'source> Wonders<'view, 'source> {
    /// Iterate over all the names of wonders owned by the character.
    pub fn iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        self.0.wonders_iter()
    }

    /// Get a specific wonder by its name.
    pub fn get(&self, name: &str) -> Option<OwnedWonder<'source>> {
        self.0.get_wonder(name)
    }
}
