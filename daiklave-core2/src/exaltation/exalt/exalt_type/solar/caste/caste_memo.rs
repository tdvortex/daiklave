use serde::{Deserialize, Serialize};

use super::{
    dawn::DawnMemo, eclipse::EclipseMemo, night::NightMemo, twilight::TwilightMemo,
    zenith::ZenithMemo, SolarCasteView,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SolarCasteMemo {
    Dawn(DawnMemo),
    Zenith(ZenithMemo),
    Twilight(TwilightMemo),
    Night(NightMemo),
    Eclipse(EclipseMemo),
}

impl<'source> SolarCasteMemo {
    pub fn as_ref(&'source self) -> SolarCasteView {
        match self {
            SolarCasteMemo::Dawn(memo) => SolarCasteView::Dawn(memo.as_ref()),
            SolarCasteMemo::Zenith(memo) => SolarCasteView::Zenith(memo.as_ref()),
            SolarCasteMemo::Twilight(memo) => SolarCasteView::Twilight(memo.as_ref()),
            SolarCasteMemo::Night(memo) => SolarCasteView::Night(memo.as_ref()),
            SolarCasteMemo::Eclipse(memo) => SolarCasteView::Eclipse(memo.as_ref()),
        }
    }
}
