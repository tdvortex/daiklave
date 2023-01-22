use crate::{charms::Charms, Character};

impl<'view, 'source> Character<'source> {
    /// Read the Charms (and Evocations and Spells) owned by the character.
    pub fn charms(&'view self) -> Charms<'view, 'source> {
        Charms(self)
    }
}
