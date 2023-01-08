mod error;
mod essence_memo;
mod mote_commitment;
mod mote_commitment_id;
mod mote_commitment_memo;
mod mote_pool;
mod mote_pool_name;
mod motes;
mod motes_memo;

use std::collections::HashMap;

pub(crate) use error::{
    CommitMotesError, RecoverMotesError, SetEssenceRatingError, SpendMotesError, UncommitMotesError,
};
pub(crate) use essence_memo::EssenceMemo;
pub use mote_commitment::MoteCommitment;
pub use mote_commitment_id::MoteCommitmentId;
pub(crate) use mote_commitment_memo::MoteCommitmentMemo;
pub use mote_pool_name::MotePoolName;
pub use motes::Motes;
pub use mote_pool::MotePool;

/// An Exalt's Essence rating and mote pools.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Essence<'source> {
    rating: u8,
    motes: Motes<'source>,
}

impl<'source> Essence<'source> {
    pub(in crate::exaltation::exalt) fn as_memo(&self) -> EssenceMemo {
        EssenceMemo::new(self.rating, self.motes.as_memo())
    }

    /// The Exalt's current Essence rating.
    pub fn rating(&self) -> u8 {
        self.rating
    }

    pub(in crate::exaltation::exalt) fn rating_mut(&mut self) -> &mut u8 {
        &mut self.rating
    }

    /// The current state of the Exalt's mote pools.
    pub fn motes(&self) -> &Motes {
        &self.motes
    }

    pub(in crate::exaltation::exalt) fn motes_mut(&mut self) -> &mut Motes<'source> {
        &mut self.motes
    }

    pub(in crate::exaltation) fn new_solar(rating: u8) -> Self {
        Self {
            rating,
            motes: Motes::new(
                MotePool::new(rating * 7 + 26, 0),
                MotePool::new(rating * 3 + 10, 0),
                HashMap::new(),
            ),
        }
    }
}
