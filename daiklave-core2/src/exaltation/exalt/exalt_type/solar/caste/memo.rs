use serde::{Deserialize, Serialize};

use super::{
    eclipse::EclipseMemo, dawn::Dawn, zenith::Zenith, twilight::Twilight, night::Night,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SolarCasteMemo {
    Dawn(Dawn),
    Zenith(Zenith),
    Twilight(Twilight),
    Night(Night),
    Eclipse(EclipseMemo),
}