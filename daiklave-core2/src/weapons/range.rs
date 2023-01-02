pub(in crate::weapons) enum WeaponRange {
    ContactOnly,
    Throwable(RangeBand),
    Archery(RangeBand),
}

pub(in crate::weapons) enum RangeBand {
    Close,
    Short,
    Medium,
    Long,
    Extreme
}