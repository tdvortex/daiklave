use crate::sorcery::SorceryArchetypeName;

use super::SorceryArchetypeMerit;

pub type AddSorceryArchetypeMerit = (SorceryArchetypeName, String, SorceryArchetypeMerit);

pub struct AddSorceryArchetypeMerit {
    archetype_name: SorceryArchetypeName,
}