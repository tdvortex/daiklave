mod eclipse;
mod inner;
mod noneclipse;
mod id;

pub use id::SpiritCharmId;

pub use eclipse::EclipseCharm;
pub use noneclipse::_NonEclipseCharm;

pub enum _SpiritCharm {
    Eclipse(EclipseCharm),
    NonEclipse(_NonEclipseCharm)
}