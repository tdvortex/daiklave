mod wonder;

pub use wonder::{OwnedWonder, Wonder, WonderId};
pub(crate) use wonder::{WonderNoAttunement, WonderNoAttunementMemo};

use crate::exaltation::Exaltation;

/// A character's collection of non-armor, non-weapon, non-Warstrider
/// artifacts.
pub struct Wonders<'view, 'source>(pub(crate) &'view Exaltation<'source>);

impl<'view, 'source> Wonders<'view, 'source> {
    /// Iterate over all the Ids for wonders owned by the character.
    pub fn iter(&self) -> impl Iterator<Item = WonderId> + '_ {
        self.0.wonders_iter()
    }

    /// Get a specific wonder by its Id.
    pub fn get(&self, wonder_id: WonderId) -> Option<OwnedWonder<'source>> {
        self.0.get_wonder(wonder_id)
    }
}
