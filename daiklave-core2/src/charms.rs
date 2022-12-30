use serde::{Deserialize, Serialize};

/// All the keywords that may be associated with a specific Charm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum CharmKeyword {
    /// Air Dragon-Blooded Aura
    Air,
    /// Aggravated damage-dealing
    Aggravated,
    /// Lunar Archetype
    Archetype,
    /// Relies on Dragon-Blooded Aura
    Aura,
    /// Balanced elemental energy for Dragon-Blooded aura
    Balanced,
    /// Bridge, alternate purchase costs
    Bridge,
    /// Affects clash attacks
    Clash,
    /// Allows or applies to a Counterattack
    Counterattack,
    /// Only applies to Decisive attacks
    DecisiveOnly,
    /// Dual (different applications to Decisive and Withering)
    Dual,
    /// An Excellency Charm
    Excellency,
    /// Fire Dragon-Blooded Aura
    Fire,
    /// Earth Dragon-Blooded Aura
    Earth,
    /// Does not increase Anima when used
    Mute,
    /// Applies when captaining a ship or boat
    Pilot,
    /// Relates to Lunar shapeshifting
    Protean,
    /// Mind-affecting Charms
    Psyche,
    /// Cannot be used while crashed
    Perilous,
    /// Requires a ritual to cast
    Ritual,
    /// Charm requires spending Crafting experience
    Salient,
    /// A Dragon-Blooded signature Elemental Charm for an Ability
    Signature,
    /// Can be applied multiple times
    Stackable,
    /// Affects Withering and Decisive attacks equally
    Uniform,
    /// Water Dragon-Blooded Aura
    Water,
    /// Applies to Withering attacks only
    WitheringOnly,
    /// Wood Dragon-Blooded Aura
    Wood,
    /// Only usable in written form
    WrittenOnly,
}

/// The various types of costs that must be paid to use a Charm (or Spell)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum CharmCostType {
    /// Spend or commit motes
    Motes,
    /// Shape Sorcery for a sufficient quantity of Sorcerous Motes
    SorcerousMotes,
    /// Spend Willpower
    Willpower,
    /// Take Bashing damage
    BashingHealth,
    /// Take Lethal damage
    LethalHealth,
    /// Take Aggravated damage
    AggravatedHealth,
    /// Burn Anima levels
    AnimaLevels,
    /// Spend Initiative
    Initiative,
    /// Spend Experience
    Experience,
    /// Spend silver craft experience
    SilverCraftExperience,
    /// Spend gold craft experience
    GoldCraftExperience,
    /// Spend white craft experience
    WhiteCraftExperience,
}

/// A cost to use a Charm, including both type and amount
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharmCost {
    cost_type: CharmCostType,
    amount: u8,
}

/// The type of action necessary to use the Charm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharmActionType {
    /// Requires a combat action to use
    Simple,
    /// Used as part of a different action (like an attack)
    Supplemental,
    /// Usable without spending an action, possibly out-of-turn
    Reflexive,
    /// No action, effect is always active
    Permanent,
}
