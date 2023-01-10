mod memo;
pub(crate) use memo::HearthstoneOriginMemo;

pub(crate) enum HearthstoneOrigin<'source> {
    // Manse is required
    Linked(&'source str),
    // Manse is optional
    ManseBorn(Option<&'source str>),
    // Manse is optional
    ManseBornSteady(Option<&'source str>),
    // Manse is optional
    Steady(Option<&'source str>),
    // Manse is not allowed
    WildBorn,
    // Manse is optional
    Unspecified(Option<&'source str>)
}