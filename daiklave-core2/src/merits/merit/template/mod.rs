mod id;
pub use id::MeritTemplateId;

mod with_dots;
pub(crate) use with_dots::{MeritTemplateWithDots, MeritTemplateWithDotsMemo};

use crate::book_reference::BookReference;

use super::MeritType;

pub struct MeritTemplate {
    name: String,
    book_reference: Option<BookReference>,
    merit_type: MeritType,
    shared_description: String,
    zero_dot_description: Option<String>,
    one_dot_description: Option<String>,
    two_dot_description: Option<String>,
    three_dot_description: Option<String>,
    four_dot_description: Option<String>,
    five_dot_description: Option<String>,
}