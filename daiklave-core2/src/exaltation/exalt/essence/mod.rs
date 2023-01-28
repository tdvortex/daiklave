mod error;
mod mote_commitment;
mod mote_pool;
mod motes;
mod set_rating;
mod state;
pub(crate) use state::{EssenceState, EssenceStateMemo};

pub(crate) use error::EssenceError;
pub use motes::{Motes, CommitMotes, RecoverMotes, SpendMotes, UncommitMotes};
pub(crate) use motes::{MotesState};

use super::{AnimaEffect, Exalt};
pub use mote_commitment::{MoteCommitment, MoteCommitmentName, MoteCommitmentNameMutation};
pub(crate) use mote_pool::MotePool;
pub use mote_pool::MotePoolName;
pub use set_rating::SetEssenceRating;

/// An Exalt's Essence rating and mote pools.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Essence<'view, 'source>(pub(crate) &'view Exalt<'source>);

impl<'view, 'source> Essence<'view, 'source> {
    /// The Exalt's current Essence rating.
    pub fn rating(&self) -> u8 {
        self.0.essence.rating.get()
    }

    /// The current state of the Exalt's mote pools.
    pub fn motes(&self) -> Motes<'view, 'source> {
        Motes {
            state: &self.0.essence.motes,
            weapons: &self.0.weapons,
            armor: &self.0.armor,
            wonders: &self.0.wonders,
        }
    }

    /// The anima effects the Exalt possesses.
    pub fn anima_effects(&self) -> impl Iterator<Item = AnimaEffect> {
        self.0.anima_effects()
    }
}
