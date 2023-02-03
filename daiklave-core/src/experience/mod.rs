mod error;
mod experience_pool;
mod gain;
mod gain_exalt;
mod spend;
mod spend_exalt;
pub use error::ExperienceError;
pub use experience_pool::ExperiencePool;
pub use gain::GainExperience;
pub use gain_exalt::GainExaltExperience;
pub use spend::SpendExperience;
pub use spend_exalt::SpendExaltExperience;

/// A character's Experience points
pub struct Experience {
    pub(crate) base: ExperiencePool,
    pub(crate) exalt: Option<ExperiencePool>,
}

impl Experience {
    /// A character's normal experience pool.
    pub fn base(&self) -> ExperiencePool {
        self.base
    }

    /// If the character is an exalt, their Exalt experience pool.
    pub fn exalt(&self) -> Option<ExperiencePool> {
        self.exalt
    }
}
