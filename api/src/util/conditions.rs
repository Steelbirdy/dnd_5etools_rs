use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[bitflags]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Condition {
    Blinded,
    Charmed,
    Deafened,
    Exhaustion,
    Frightened,
    Grappled,
    Incapacitated,
    Invisible,
    Paralyzed,
    Petrified,
    Poisoned,
    Prone,
    Restrained,
    Stunned,
    Unconscious,
    Disease,
}

pub type ConditionImmunityArray<'a> = Option<Vec<ConditionImmunity<'a>>>;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConditionImmunity<'a> {
    Simple(Condition),
    Special {
        special: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    Annotated {
        pre_note: Option<&'a str>,
        condition_immune: ConditionImmunityArray<'a>,
        note: Option<&'a str>,
    },
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct TagsConditions(
    #[serde(with = "crate::serde_utils::bitflags_as_seq")] pub BitFlags<Condition>,
);
