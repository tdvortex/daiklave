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
    Unspecified(Option<(String, String)>)
}