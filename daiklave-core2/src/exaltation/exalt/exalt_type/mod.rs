/// Traits for the Solar Exalted
pub mod solar;

mod exalt_type_memo;

pub(crate) use exalt_type_memo::ExaltTypeMemo;

use crate::artifact::{MagicMaterial, Sonance};

use self::solar::Solar;

/// A particular type of Exalt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExaltType<'source> {
    /// The Solar Exalted, chosen of the Unconquered Sun.
    Solar(Solar<'source>),
}

impl<'source> ExaltType<'source> {
    pub(crate) fn as_memo(&self) -> ExaltTypeMemo {
        match self {
            ExaltType::Solar(view) => ExaltTypeMemo::Solar(view.as_memo()),
        }
    }

    pub(crate) fn sonance(&self, magic_material: MagicMaterial) -> Option<Sonance> {
        match (self, magic_material) {
            (ExaltType::Solar(_), _) => Some(Sonance::Resonant),
        }
    }
}
