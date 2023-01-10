use serde::{Deserialize, Serialize};

use super::HearthstoneOrigin;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum HearthstoneOriginMemo {
    // Manse is required
    Linked((String, String)),
    // Manse is optional
    ManseBorn(Option<(String, String)>),
    // Manse is optional
    ManseBornSteady(Option<(String, String)>),
    // Manse is optional
    Steady(Option<(String, String)>),
    // Manse is not allowed
    WildBorn,
    // Manse is optional
    Unspecified(Option<(String, String)>),
}

impl<'source> HearthstoneOriginMemo {
    pub fn as_ref(&'source self) -> HearthstoneOrigin {
        match self {
            HearthstoneOriginMemo::Linked((manse, demense)) => {
                HearthstoneOrigin::Linked((manse.as_str(), demense.as_str()))
            }
            HearthstoneOriginMemo::ManseBorn(maybe_m_and_d) => HearthstoneOrigin::ManseBorn(
                maybe_m_and_d
                    .as_ref()
                    .map(|m_and_d| (m_and_d.0.as_str(), m_and_d.1.as_str())),
            ),
            HearthstoneOriginMemo::ManseBornSteady(maybe_m_and_d) => {
                HearthstoneOrigin::ManseBornSteady(
                    maybe_m_and_d
                        .as_ref()
                        .map(|m_and_d| (m_and_d.0.as_str(), m_and_d.1.as_str())),
                )
            }
            HearthstoneOriginMemo::Steady(maybe_m_and_d) => HearthstoneOrigin::Steady(
                maybe_m_and_d
                    .as_ref()
                    .map(|m_and_d| (m_and_d.0.as_str(), m_and_d.1.as_str())),
            ),
            HearthstoneOriginMemo::WildBorn => HearthstoneOrigin::WildBorn,
            HearthstoneOriginMemo::Unspecified(maybe_m_and_d) => HearthstoneOrigin::Unspecified(
                maybe_m_and_d
                    .as_ref()
                    .map(|m_and_d| (m_and_d.0.as_str(), m_and_d.1.as_str())),
            ),
        }
    }
}
