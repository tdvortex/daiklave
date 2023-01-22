use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

use serde::{Deserialize, Serialize};

use crate::{
    book_reference::BookReference,
    charms::{CharmActionType, CharmCost, CharmCostType},
};

/// A builder path to construct an Evocation.
pub mod builder;
mod evokable_id;
mod id;
mod keyword;
pub use evokable_id::EvokableId;
pub use id::EvocationId;
pub use keyword::EvocationKeyword;

use self::builder::EvocationBuilder;

use super::CharmId;

/// A Charm which is drawn from the unique power of a Hearthstone or named
/// Artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evocation {
    evokable_id: EvokableId,
    book_reference: Option<BookReference>,
    name: String,
    summary: Option<String>,
    description: String,
    resonant: Option<String>,
    dissonant: Option<String>,
    essence_required: NonZeroU8,
    evocation_tree: HashSet<EvocationId>,
    upgrade_charm: Option<CharmId>,
    keywords: HashSet<EvocationKeyword>,
    costs: HashMap<CharmCostType, NonZeroU8>,
    action_type: CharmActionType,
    duration: String,
}

impl Evocation {
    /// Starts a builder for a new Evocation.
    pub fn builder(evokable_id: EvokableId, name: String) -> EvocationBuilder {
        EvocationBuilder {
            name,
            book_reference: None,
            summary: None,
            evokable_id,
            resonant: None,
            dissonant: None,
            evocation_tree: HashSet::new(),
            upgrade_charm: None,
            keywords: HashSet::new(),
            costs: HashMap::new(),
        }
    }
}

impl Evocation {
    /// The Id of the Artifact or Hearthstone this Evocation is drawn from.
    pub fn evokable_id(&self) -> EvokableId {
        self.evokable_id
    }

    /// The book reference for this Evocation.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    /// The name of this Evocation.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// A short summary of this Evocation's effects.
    pub fn summary(&self) -> Option<&str> {
        self.summary.as_deref()
    }

    /// The full description of this Evocation's effects, excluding resonant
    /// and dissonant modifiers.
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// A description of the extra effects of this Evocation if the Exalt is
    /// resonant with the artifact's Magic Material.
    pub fn resonant(&self) -> Option<&str> {
        self.resonant.as_deref()
    }

    /// A description of the reduced effects of this Evocation if the Exalt is
    /// dissonant with the artifact's Magic Material.
    pub fn dissonant(&self) -> Option<&str> {
        self.dissonant.as_deref()
    }

    /// The Essence level required to learn this Evocation.
    pub fn essence_required(&self) -> u8 {
        self.essence_required.get()
    }

    /// The other Evocations (typically on the same Artifact/Hearthstone)
    /// which the Exalt must have to purchase this Charm.
    pub fn evocation_prerequisites(&self) -> impl Iterator<Item = EvocationId> + '_ {
        self.evocation_tree.iter().copied()
    }

    /// If the Evocation is an upgrade to a non-Evocation Charm, the Id of that
    /// Charm.
    pub fn upgrade(&self) -> Option<CharmId> {
        self.upgrade_charm
    }

    /// Any keywords the Evocation possesses.
    pub fn keywords(&self) -> impl Iterator<Item = EvocationKeyword> + '_ {
        let mut output = self
            .keywords
            .iter()
            .copied()
            .collect::<Vec<EvocationKeyword>>();
        output.sort();
        output.into_iter()
    }

    /// The costs required to activate the Evocation (not counting attunement).
    pub fn costs(&self) -> impl Iterator<Item = CharmCost> + '_ {
        let mut output = self
            .costs
            .iter()
            .map(|(cost_type, amount)| CharmCost::new(*cost_type, amount.get()))
            .collect::<Vec<CharmCost>>();
        output.sort();
        output.into_iter()
    }
}
