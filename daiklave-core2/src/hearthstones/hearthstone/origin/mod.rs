mod memo;
pub(crate) use memo::HearthstoneOriginMemo;

use super::keyword::HearthstoneKeyword;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HearthstoneOrigin<'source> {
    // Manse is required
    Linked((&'source str, &'source str)),
    // Manse is optional
    ManseBorn(Option<(&'source str, &'source str)>),
    // Manse is optional
    ManseBornSteady(Option<(&'source str, &'source str)>),
    // Manse is optional
    Steady(Option<(&'source str, &'source str)>),
    // Manse is not allowed
    WildBorn,
    // Manse is optional
    Unspecified(Option<(&'source str, &'source str)>),
}

impl<'source> From<&'source HearthstoneOriginMemo> for HearthstoneOrigin<'source> {
    fn from(memo: &'source HearthstoneOriginMemo) -> Self {
        match memo {
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

impl<'source> HearthstoneOrigin<'source> {
    pub fn keywords(&self) -> impl Iterator<Item = HearthstoneKeyword> {
        match self {
            HearthstoneOrigin::Linked(_) => vec![HearthstoneKeyword::Linked],
            HearthstoneOrigin::ManseBorn(_) => vec![HearthstoneKeyword::ManseBorn],
            HearthstoneOrigin::ManseBornSteady(_) => {
                vec![HearthstoneKeyword::ManseBorn, HearthstoneKeyword::Steady]
            }
            HearthstoneOrigin::Steady(_) => vec![HearthstoneKeyword::Steady],
            HearthstoneOrigin::WildBorn => vec![HearthstoneKeyword::WildBorn],
            HearthstoneOrigin::Unspecified(_) => vec![],
        }
        .into_iter()
    }

    pub fn manse_and_demense(&self) -> Option<(&'source str, &'source str)> {
        match self {
            HearthstoneOrigin::Linked(m_and_d) => Some(*m_and_d),
            HearthstoneOrigin::ManseBorn(m_and_d) => m_and_d.as_ref().copied(),
            HearthstoneOrigin::ManseBornSteady(m_and_d) => m_and_d.as_ref().copied(),
            HearthstoneOrigin::Steady(m_and_d) => m_and_d.as_ref().copied(),
            HearthstoneOrigin::WildBorn => None,
            HearthstoneOrigin::Unspecified(m_and_d) => m_and_d.as_ref().copied(),
        }
    }
}
