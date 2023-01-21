mod eclipse;
mod inner;
mod noneclipse;

pub use eclipse::EclipseCharm;
pub use noneclipse::_NonEclipseCharm;

pub enum _SpiritCharm {
    Eclipse(EclipseCharm),
    NonEclipse(_NonEclipseCharm)
}