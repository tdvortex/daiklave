use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

use serde::{Deserialize, Serialize};

use crate::{
    artifact::{ArtifactName, ArtifactNameMutation},
    book_reference::BookReference,
    charms::{CharmActionType, CharmCost, CharmCostType},
};

mod add;
/// A builder path to construct an Evocation.
pub mod builder;
mod evokable_name;
mod keyword;
pub use add::AddEvocation;
pub use evokable_name::{EvokableName, EvokableNameMutation};
pub use keyword::EvocationKeyword;

use self::builder::EvocationBuilder;

use super::{CharmId, CharmName};

/// A Charm which is drawn from the unique power of a Hearthstone or named
/// Artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evocation {
    evokable_name: EvokableNameMutation,
    book_reference: Option<BookReference>,
    summary: Option<String>,
    description: String,
    resonant: Option<String>,
    dissonant: Option<String>,
    essence_required: NonZeroU8,
    evocation_tree: HashSet<String>,
    upgrade_charm: Option<CharmName>,
    keywords: HashSet<EvocationKeyword>,
    costs: HashMap<CharmCostType, NonZeroU8>,
    action_type: CharmActionType,
    duration: String,
}

impl Evocation {
    /// Starts a builder for a new Evocation.
    pub fn builder(evokable_name: EvokableNameMutation, name: String) -> EvocationBuilder {
        EvocationBuilder {
            name,
            book_reference: None,
            summary: None,
            evokable_name,
            resonant: None,
            dissonant: None,
            evocation_tree: HashSet::new(),
            upgrade_charm: None,
            keywords: HashSet::new(),
            costs: HashMap::new(),
        }
    }
}

impl<'source> Evocation {
    /// The name of the Artifact or Hearthstone this Evocation is drawn from.
    pub fn evokable_name(&'source self) -> EvokableName<'source> {
        match &self.evokable_name {
            EvokableNameMutation::Hearthstone(hearthstone_name) => {
                EvokableName::Hearthstone(hearthstone_name.as_str())
            }
            EvokableNameMutation::Artifact(ArtifactNameMutation::Weapon(weapon_name)) => {
                EvokableName::Artifact(ArtifactName::Weapon(weapon_name.as_str()))
            }
            EvokableNameMutation::Artifact(ArtifactNameMutation::Armor(armor_name)) => {
                EvokableName::Artifact(ArtifactName::Armor(armor_name.as_str()))
            }
            EvokableNameMutation::Artifact(ArtifactNameMutation::Wonder(wonder_name)) => {
                EvokableName::Artifact(ArtifactName::Wonder(wonder_name.as_str()))
            }
        }
    }

    /// The book reference for this Evocation.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
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
    pub fn evocation_prerequisites(&'source self) -> impl Iterator<Item = &'source str> + '_ {
        self.evocation_tree.iter().map(|s| s.as_str())
    }

    /// If the Evocation is an upgrade to a non-Evocation Charm, the Id of that
    /// Charm.
    pub fn upgrade(&'source self) -> Option<CharmId<'source>> {
        match &self.upgrade_charm {
            Some(charm_name) => Some(match charm_name {
                CharmName::Spirit(spirit_id) => CharmId::Spirit(*spirit_id),
                CharmName::Evocation(evocation_name) => CharmId::Evocation(evocation_name.as_str()),
                CharmName::MartialArts(martial_arts_charm_name) => {
                    CharmId::MartialArts(martial_arts_charm_name.as_str())
                }
                CharmName::Solar(solar_id) => CharmId::Solar(solar_id.as_str()),
                CharmName::Spell(spell_name) => CharmId::Spell(spell_name.as_str()),
            }),
            None => None,
        }
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
