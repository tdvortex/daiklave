/// Dawn Caste, warriors and generals
pub mod dawn;

/// Eclipse Caste, dealmakers and travelers
pub mod eclipse;

/// Night Caste, rogues and swashbucklers
pub mod night;

/// Twilight Caste, scholars and sorcerers
pub mod twilight;

/// Zenith Caste, leaders and priests
pub mod zenith;

mod caste_memo;
mod caste_view;

pub(crate) use caste_memo::SolarCasteMemo;
pub(crate) use caste_view::SolarCasteView;
