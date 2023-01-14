use thiserror::Error;

/// An error specifically related to a Solar's trait layout.
#[derive(Debug, Error)]
pub enum SolarError {
    /// All Solars must have a Supernal ability
    #[error("Supernal ability is required")]
    SupernalRequired,
    /// All Solars must have exactly 5 unique Caste abilities, one of which is
    /// their Supernal ability (or Brawl, if Martial Arts is Supernal).
    #[error("Need exactly 5 caste abilities")]
    FiveCasteAbilities,
    /// All Solars must have exactly 5 unique Favored abilities. Martial Arts
    /// cannot be Favored.
    #[error("Need exactly 5 favored abilities")]
    FiveFavoredAbilities,
    /// Caste and Favored abilities must be distinct, non-overlapping groups.
    #[error("Can't have an ability as both Caste and Favored")]
    CasteAndFavoredUnique,
    /// Solars require a Limit Trigger.
    #[error("Limit Trigger is required")]
    LimitTriggerRequired,
    /// Supernal abilities must also be Caste abilities
    #[error("Supernal abilities must be Caste")]
    SupernalIsCaste,
}
