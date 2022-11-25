#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum RangeBand {
    Close,
    Short,
    Medium,
    Long,
    Extreme,
}
