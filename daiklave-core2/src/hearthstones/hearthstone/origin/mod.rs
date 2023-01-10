mod memo;
pub(crate) use memo::HearthstoneOriginMemo;

use crate::hearthstones::keyword::HearthstoneKeyword;

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

impl<'source> HearthstoneOrigin<'source> {
    pub fn as_memo(&self) -> HearthstoneOriginMemo {
        match self {
            HearthstoneOrigin::Linked(m_and_d) => {
                HearthstoneOriginMemo::Linked((m_and_d.0.to_owned(), m_and_d.1.to_owned()))
            }
            HearthstoneOrigin::ManseBorn(maybe_m_and_d) => HearthstoneOriginMemo::ManseBorn(
                maybe_m_and_d.map(|m_and_d| (m_and_d.0.to_owned(), m_and_d.1.to_owned())),
            ),
            HearthstoneOrigin::ManseBornSteady(maybe_m_and_d) => {
                HearthstoneOriginMemo::ManseBornSteady(
                    maybe_m_and_d.map(|m_and_d| (m_and_d.0.to_owned(), m_and_d.1.to_owned())),
                )
            }
            HearthstoneOrigin::Steady(maybe_m_and_d) => HearthstoneOriginMemo::Steady(
                maybe_m_and_d.map(|m_and_d| (m_and_d.0.to_owned(), m_and_d.1.to_owned())),
            ),
            HearthstoneOrigin::WildBorn => HearthstoneOriginMemo::WildBorn,
            HearthstoneOrigin::Unspecified(maybe_m_and_d) => HearthstoneOriginMemo::Unspecified(
                maybe_m_and_d.map(|m_and_d| (m_and_d.0.to_owned(), m_and_d.1.to_owned())),
            ),
        }
    }

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
            HearthstoneOrigin::ManseBorn(m_and_d) => m_and_d.as_ref().map(|pair| *pair),
            HearthstoneOrigin::ManseBornSteady(m_and_d) => m_and_d.as_ref().map(|pair| *pair),
            HearthstoneOrigin::Steady(m_and_d) => m_and_d.as_ref().map(|pair| *pair),
            HearthstoneOrigin::WildBorn => None,
            HearthstoneOrigin::Unspecified(m_and_d) => m_and_d.as_ref().map(|pair| *pair),
        }
    }
}
