/// Traits for the Solar Exalted
pub mod solar;

mod exalt_type_memo;

pub(crate) use exalt_type_memo::ExaltTypeMemo;

use crate::artifact::{MagicMaterial, Sonance};

use self::solar::Solar;

use super::Limit;

/// A particular type of Exalt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExaltType<'source> {
    /// The Solar Exalted, chosen of the Unconquered Sun.
    Solar(Solar<'source>),
}

impl<'source> ExaltType<'source> {
    pub(crate) fn sonance(&self, magic_material: MagicMaterial) -> Option<Sonance> {
        match (self, magic_material) {
            (ExaltType::Solar(_), _) => Some(Sonance::Resonant),
        }
    }

    pub(crate) fn limit(&self) -> Option<Limit<'source>> {
        match self {
            ExaltType::Solar(solar) => Some(solar.limit()),
        }
    }

    pub(crate) fn limit_mut(&mut self) -> Option<&mut Limit<'source>> {
        match self {
            ExaltType::Solar(solar) => Some(&mut solar.limit),
        }
    }
}
