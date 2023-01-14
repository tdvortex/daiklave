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

pub struct SolarBuilder {
    pub(crate) limit_trigger: Option<String>,
}

impl SolarBuilder {
    pub fn limit_trigger(mut self, limit_trigger: String) -> Self {
        self.limit_trigger = Some(limit_trigger);
        self
    }

    pub fn dawn(self) -> DawnBuilder {
        DawnBuilder {
            caste_abilities: Vec::new(),
            supernal_ability: None,
            favored_abilities: Vec::new(),
            limit_trigger: self.limit_trigger,
        }
    }

    pub fn zenith(self) -> ZenithBuilder {
        ZenithBuilder {
            caste_abilities: Vec::new(),
            supernal_ability: None,
            favored_abilities: Vec::new(),
            limit_trigger: self.limit_trigger,
        }
    }

    pub fn twilight(self) -> TwilightBuilder {
        TwilightBuilder {
            caste_abilities: Vec::new(),
            supernal_ability: None,
            favored_abilities: Vec::new(),
            limit_trigger: self.limit_trigger,
        }
    }

    pub fn night(self) -> NightBuilder {
        NightBuilder {
            caste_abilities: Vec::new(),
            supernal_ability: None,
            favored_abilities: Vec::new(),
            limit_trigger: self.limit_trigger,
        }
    }

    pub fn eclipse(self) -> EclipseBuilder {
        EclipseBuilder {
            caste_abilities: Vec::new(),
            supernal_ability: None,
            favored_abilities: Vec::new(),
            limit_trigger: self.limit_trigger,
        }
    }
}
