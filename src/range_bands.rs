#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum RangeBand {
    Close,
    Short,
    Medium,
    Long,
    Extreme,
}