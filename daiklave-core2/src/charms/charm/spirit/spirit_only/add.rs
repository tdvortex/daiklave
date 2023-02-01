use crate::charms::charm::spirit::SpiritCharmName;

use super::SpiritOnlyCharm;

/// A charm which can only be added to non-Exalts.
pub struct AddSpiritOnlyCharm {
    pub(crate) _name: SpiritCharmName,
    pub(crate) _charm: SpiritOnlyCharm,
}