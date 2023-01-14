use crate::exaltation::exalt::{ExaltArmor, ExaltWeapons, ExaltWonders};

mod state;
pub(crate) use state::{MotesState, MotesStateMemo};

use super::{MoteCommitment, MoteCommitmentId, MotePool};

/// The current status of an Exalt's motes of Essence.
pub struct Motes<'view, 'source> {
    pub(crate) state: &'view MotesState<'source>,
    pub(crate) weapons: &'view ExaltWeapons<'source>,
    pub(crate) armor: &'view ExaltArmor<'source>,
    pub(crate) wonders: &'view ExaltWonders<'source>,
}

impl<'view, 'source> Motes<'view, 'source> {
    /// The Exalt's peripheral motes.
    pub fn peripheral(&self) -> &'view MotePool {
        self.state.peripheral()
    }

    /// The Exalt's personal motes.
    pub fn personal(&self) -> &'view MotePool {
        self.state.personal()
    }

    /// All effects the Exalt has currently committed motes to (including
    /// artifact attunement)
    pub fn committed(&self) -> impl Iterator<Item = (MoteCommitmentId, MoteCommitment<'source>)> {
        vec![].into_iter()
    }
}
