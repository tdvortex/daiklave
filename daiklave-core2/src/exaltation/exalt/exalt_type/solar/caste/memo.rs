use serde::{Deserialize, Serialize};

use super::{dawn::Dawn, eclipse::EclipseMemo, night::Night, twilight::Twilight, zenith::Zenith};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SolarCasteMemo {
    Dawn(Dawn),
    Zenith(Zenith),
    Twilight(Twilight),
    Night(Night),
    Eclipse(EclipseMemo),
}
