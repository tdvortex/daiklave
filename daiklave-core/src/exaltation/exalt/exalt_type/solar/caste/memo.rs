use serde::{Deserialize, Serialize};

use super::{dawn::Dawn, eclipse::EclipseMemo, night::Night, twilight::Twilight, zenith::Zenith, SolarCaste};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SolarCasteMemo {
    Dawn(Dawn),
    Zenith(Zenith),
    Twilight(Twilight),
    Night(Night),
    Eclipse(EclipseMemo),
}

impl From<&SolarCaste<'_>> for SolarCasteMemo {
    fn from(value: &SolarCaste<'_>) -> Self {
        match value {
            SolarCaste::Dawn(dawn) => Self::Dawn(dawn.to_owned()),
            SolarCaste::Zenith(zenith) => Self::Zenith(zenith.to_owned()),
            SolarCaste::Twilight(twilight) => Self::Twilight(twilight.to_owned()),
            SolarCaste::Night(night) => Self::Night(night.to_owned()),
            SolarCaste::Eclipse(eclipse) => Self::Eclipse(eclipse.into()),
        }
    }
}