use serde::{Deserialize, Serialize};

use super::{
    dawn::DawnMemo, eclipse::EclipseMemo, night::NightMemo, twilight::TwilightMemo,
    zenith::ZenithMemo, SolarCaste,
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
    pub fn as_ref(&'source self) -> SolarCaste {
        match self {
            SolarCasteMemo::Dawn(memo) => SolarCaste::Dawn(memo.as_ref()),
            SolarCasteMemo::Zenith(memo) => SolarCaste::Zenith(memo.as_ref()),
            SolarCasteMemo::Twilight(memo) => SolarCaste::Twilight(memo.as_ref()),
            SolarCasteMemo::Night(memo) => SolarCaste::Night(memo.as_ref()),
            SolarCasteMemo::Eclipse(memo) => SolarCaste::Eclipse(memo.as_ref()),
        }
    }
}
