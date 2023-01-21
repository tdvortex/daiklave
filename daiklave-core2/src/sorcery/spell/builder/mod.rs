use crate::book_reference::BookReference;

/// Builder for a Spell.
pub struct SpellBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
}
