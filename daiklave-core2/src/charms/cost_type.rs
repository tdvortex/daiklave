use serde::{Deserialize, Serialize};

/// The various types of costs that must be paid to use a Charm (or Spell)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum CharmCostType {
    /// Spend or commit motes
    Motes,
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
