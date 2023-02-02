use serde::{Deserialize, Serialize};

use crate::merits::merit::{manse::ManseName, DemenseName};

use super::HearthstoneOrigin;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum HearthstoneOriginMemo {
    // Manse is required
    Linked((ManseName, DemenseName)),
    // Manse is optional
    ManseBorn(Option<(ManseName, DemenseName)>),
    // Manse is optional
    ManseBornSteady(Option<(ManseName, DemenseName)>),
    // Manse is optional
    Steady(Option<(ManseName, DemenseName)>),
    // Manse is not allowed
    WildBorn,
    // Manse is optional
    Unspecified(Option<(ManseName, DemenseName)>),
}

impl From<&HearthstoneOrigin<'_>> for HearthstoneOriginMemo {
    fn from(view: &HearthstoneOrigin<'_>) -> Self {
        match view {
            HearthstoneOrigin::Linked(m_and_d) => {
                HearthstoneOriginMemo::Linked((m_and_d.0.into(), m_and_d.1.into()))
            }
            HearthstoneOrigin::ManseBorn(maybe_m_and_d) => HearthstoneOriginMemo::ManseBorn(
                maybe_m_and_d.map(|m_and_d| (m_and_d.0.into(), m_and_d.1.into())),
            ),
            HearthstoneOrigin::ManseBornSteady(maybe_m_and_d) => {
                HearthstoneOriginMemo::ManseBornSteady(
                    maybe_m_and_d.map(|m_and_d| (m_and_d.0.into(), m_and_d.1.into())),
                )
            }
            HearthstoneOrigin::Steady(maybe_m_and_d) => HearthstoneOriginMemo::Steady(
                maybe_m_and_d.map(|m_and_d| (m_and_d.0.into(), m_and_d.1.into())),
            ),
            HearthstoneOrigin::WildBorn => HearthstoneOriginMemo::WildBorn,
            HearthstoneOrigin::Unspecified(maybe_m_and_d) => HearthstoneOriginMemo::Unspecified(
                maybe_m_and_d.map(|m_and_d| (m_and_d.0.into(), m_and_d.1.into())),
            ),
        }
    }
}
