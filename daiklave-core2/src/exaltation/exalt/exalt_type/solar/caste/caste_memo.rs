use serde::{Deserialize, Serialize};

use super::{dawn::DawnMemo, eclipse::EclipseMemo, night::NightMemo, twilight::TwilightMemo, zenith::ZenithMemo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SolarCasteMemo {
    Dawn(DawnMemo),
    Zenith(ZenithMemo),
    Twilight(TwilightMemo),
    Night(NightMemo),
    Eclipse(EclipseMemo),
}