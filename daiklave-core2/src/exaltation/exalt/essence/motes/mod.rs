mod commit;
mod recover;
mod spend;
mod state;
mod uncommit;
pub use commit::CommitMotes;
pub use recover::RecoverMotes;
pub use spend::SpendMotes;
pub(crate) use state::{MotesState, MotesStateMemo};
pub use uncommit::UncommitMotes;

use super::{MoteCommitment, MotePool, MotePoolName};

/// The current status of an Exalt's motes of Essence.
pub struct Motes<'view, 'source> {
    pub(crate) state: &'view MotesState<'source>,
    pub(crate) attunements: Vec<MoteCommitment<'source>>,
}

impl<'view, 'source> Motes<'view, 'source> {
    /// The Exalt's peripheral motes.
    pub fn peripheral(&self) -> MotePool {
        self.peripheral_and_personal().0
    }

    /// The Exalt's personal motes.
    pub fn personal(&self) -> MotePool {
        self.peripheral_and_personal().1
    }

    /// Returns both the peripheral and personal pools simultaneously.
    pub fn peripheral_and_personal(&self) -> (MotePool<'source>, MotePool<'source>) {
        let (peripheral_commitments, personal_commitments) = self.committed().fold(
            (Vec::new(), Vec::new()),
            |(mut peripheral, mut personal), commitment| {
                if commitment.peripheral > 0 {
                    peripheral.push(commitment);
                }

                if commitment.personal > 0 {
                    personal.push(commitment);
                }
                (peripheral, personal)
            },
        );

        (
            MotePool {
                name: MotePoolName::Peripheral,
                available: self.state.peripheral_available,
                spent: self.state.peripheral_spent,
                commitments: peripheral_commitments,
            },
            MotePool {
                name: MotePoolName::Personal,
                available: self.state.personal_available,
                spent: self.state.personal_spent,
                commitments: personal_commitments,
            },
        )
    }

    /// All effects the Exalt has currently committed motes to (including
    /// artifact attunement)
    pub fn committed(&self) -> impl Iterator<Item = MoteCommitment<'source>> + '_ {
        self.attunements
            .iter()
            .copied()
            .chain(self.state.commitments())
    }
}
