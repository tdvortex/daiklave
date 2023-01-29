use super::{SpiritCharm, SpiritCharmName};

/// A Spirit charm (which may be Eclipse or not) to be added to a character.
pub struct AddSpiritCharm {
    name: SpiritCharmName,
    charm: SpiritCharm,
}