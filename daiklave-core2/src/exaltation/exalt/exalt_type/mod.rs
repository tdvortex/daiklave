/// Traits for the Solar Exalted
pub mod solar;

mod exalt_type_memo;

pub(crate) use exalt_type_memo::ExaltTypeMemo;

use crate::artifact::{MagicMaterial, Sonance};

use self::solar::Solar;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExaltType<'source> {
    Solar(Solar<'source>),
}

impl<'source> ExaltType<'source> {
    pub fn as_memo(&self) -> ExaltTypeMemo {
        match self {
            ExaltType::Solar(view) => ExaltTypeMemo::Solar(view.as_memo()),
        }
    }

    pub fn is_solar(&self) -> bool {
        true
    }

    pub fn solar_traits(&self) -> Option<&Solar> {
        match self {
            ExaltType::Solar(solar_traits) => Some(solar_traits),
        }
    }

    pub fn sonance(&self, magic_material: MagicMaterial) -> Option<Sonance> {
        match (self, magic_material) {
            (ExaltType::Solar(_), _) => Some(Sonance::Resonant),
        }
    }
}
