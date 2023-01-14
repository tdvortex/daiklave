mod dawn;
mod eclipse;
mod night;
mod twilight;
mod zenith;
pub use dawn::DawnBuilder;
pub use eclipse::EclipseBuilder;
pub use night::NightBuilder;
pub use twilight::TwilightBuilder;
pub use zenith::ZenithBuilder;

/// A builder for constructing a new Solar Exalted. Requires choosing a caste
/// first, then the caste/supernal/favored abilities for that caste. A Limit
/// Trigger must also be provided but can be be set at any time.
pub struct SolarBuilder {
    pub(crate) limit_trigger: Option<String>,
}

impl SolarBuilder {
    /// Sets the Limit Trigger for the Solar.
    pub fn limit_trigger(mut self, limit_trigger: String) -> Self {
        self.limit_trigger = Some(limit_trigger);
        self
    }

    /// Sets the Solar to be a Dawn caste.
    pub fn dawn(self) -> DawnBuilder {
        DawnBuilder {
            caste_abilities: Vec::new(),
            supernal_ability: None,
            favored_abilities: Vec::new(),
            limit_trigger: self.limit_trigger,
        }
    }

    /// Sets the Solar to be a Zenith caste.
    pub fn zenith(self) -> ZenithBuilder {
        ZenithBuilder {
            caste_abilities: Vec::new(),
            supernal_ability: None,
            favored_abilities: Vec::new(),
            limit_trigger: self.limit_trigger,
        }
    }

    /// Sets the Solar to be a Twilight caste.
    pub fn twilight(self) -> TwilightBuilder {
        TwilightBuilder {
            caste_abilities: Vec::new(),
            supernal_ability: None,
            favored_abilities: Vec::new(),
            limit_trigger: self.limit_trigger,
        }
    }

    /// Sets the Solar to be a Night caste.
    pub fn night(self) -> NightBuilder {
        NightBuilder {
            caste_abilities: Vec::new(),
            supernal_ability: None,
            favored_abilities: Vec::new(),
            limit_trigger: self.limit_trigger,
        }
    }

    /// Sets the Solar to be an Eclipse caste.
    pub fn eclipse(self) -> EclipseBuilder {
        EclipseBuilder {
            caste_abilities: Vec::new(),
            supernal_ability: None,
            favored_abilities: Vec::new(),
            limit_trigger: self.limit_trigger,
        }
    }
}
