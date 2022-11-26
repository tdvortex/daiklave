#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub enum RangeBand {
    Close,
    Short,
    Medium,
    Long,
    Extreme,
}
