use super::{solar::SolarView, ExaltTypeMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExaltTypeView<'source> {
    Solar(SolarView<'source>),
}

impl<'source> ExaltTypeView<'source> {
    pub fn as_memo(&self) -> ExaltTypeMemo {
        match self {
            ExaltTypeView::Solar(view) => ExaltTypeMemo::Solar(view.as_memo()),
        }
    }

    pub fn is_solar(&self) -> bool {
        true
    }

    pub fn solar_traits(&self) -> Option<&SolarView> {
        match self {
            ExaltTypeView::Solar(solar_traits) => Some(solar_traits),
        }
    }
}
