mod id;
pub use id::WonderId;

mod memo;
pub use memo::Wonder;

mod owned;
pub use owned::{OwnedWonder};
pub(crate) use owned::{WonderNoAttunement, WonderNoAttunementMemo};