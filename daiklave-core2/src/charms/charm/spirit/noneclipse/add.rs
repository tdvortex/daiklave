use crate::charms::charm::spirit::SpiritCharmName;

use super::NonEclipseCharm;

pub struct AddNonEclipseCharm {
    pub(crate) name: SpiritCharmName,
    pub(crate) charm: NonEclipseCharm,
}