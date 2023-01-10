pub(crate) enum HearthstoneOriginMemo {
    // Manse is required
    Linked(String),
    // Manse is optional
    ManseBorn(Option<String>),
    // Manse is optional
    ManseBornSteady(Option<String>),
    // Manse is optional
    Steady(Option<String>),
    // Manse is not allowed
    WildBorn,
    // Manse is optional
    Unspecified(Option<String>)
}