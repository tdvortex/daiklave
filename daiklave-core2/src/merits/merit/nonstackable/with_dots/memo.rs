use super::{ZeroDotsNonStackableMeritMemo, OneDotNonStackableMeritMemo, TwoDotsNonStackableMeritMemo, ThreeDotsNonStackableMeritMemo, FourDotsNonStackableMeritMemo, FiveDotsNonStackableMeritMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NonStackableMeritWithDotsMemo {
    Zero(ZeroDotsNonStackableMeritMemo),
    One(OneDotNonStackableMeritMemo),
    Two(TwoDotsNonStackableMeritMemo),
    Three(ThreeDotsNonStackableMeritMemo),
    Four(FourDotsNonStackableMeritMemo),
    Five(FiveDotsNonStackableMeritMemo),
}
