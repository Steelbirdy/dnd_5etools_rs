use crate::util::ability::Ability;
use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// A collection of additional spells which a feature grants.
#[allow(dead_code)]
pub type AdditionalSpellsArray<'a> = Vec<AdditionalSpells<'a>>;

/// At least 1 of this type's properties must be Some(_)
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalSpells<'a> {
    /// Optionally specify the ability score used for e.g. racial spellcasting
    ability: Option<AdditionalSpellAbility>,
    #[serde(borrow)]
    innate: Option<AdditionalSpellObject<'a>>,
    prepared: Option<AdditionalSpellObject<'a>>,
    expanded: Option<AdditionalSpellObject<'a>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdditionalSpellAbility {
    Abilities {
        #[serde(with = "crate::serde_utils::bitflags_as_seq")]
        choose: BitFlags<Ability>,
    },
    Inherit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalSpellObject<'a> {
    /// Patterns:
    ///  1. ^\\d+$: Spells keyed by character/class level.
    ///  2. ^s\\d+$: Spells keyed by spell level, access to which is gated behind the character advancing to a point
    ///     where they could cast those spells with e.g. a Spellcasting class feature (Mark spells from ERLW races)
    #[serde(flatten, borrow)]
    pub num_props: HashMap<u8, AdditionalSpellKind<'a>>,
    #[serde(flatten)]
    pub str_props: HashMap<&'a str, AdditionalSpellKind<'a>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdditionalSpellKind<'a> {
    #[serde(borrow)]
    StringOrChoice(AdditionalSpellArrayOfStringOrChoiceObject<'a>),
    Level(Box<AdditionalSpellLevelObject<'a>>),
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalSpellLevelObject<'a> {
    #[serde(borrow)]
    pub rest: Option<AdditionalSpellRechargeObject<'a>>,
    pub daily: Option<AdditionalSpellRechargeObject<'a>>,
    pub will: Option<AdditionalSpellArrayOfStringOrChoiceObject<'a>>,
    pub ritual: Option<AdditionalSpellArrayOfStringOrChoiceObject<'a>>,
    #[serde(rename = "_")]
    pub _other: Option<AdditionalSpellArrayOfStringOrChoiceObject<'a>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalSpellRechargeObject<'a> {
    #[serde(flatten, borrow)]
    pub num_props: HashMap<u8, AdditionalSpellArrayOfStringOrChoiceObject<'a>>,
    #[serde(flatten)]
    pub str_props: HashMap<&'a str, AdditionalSpellArrayOfStringOrChoiceObject<'a>>,
}

pub type AdditionalSpellArrayOfStringOrChoiceObject<'a> =
    Vec<AdditionalSpellStringOrChoiceObject<'a>>;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdditionalSpellStringOrChoiceObject<'a> {
    String(&'a str),
    Choice { choose: &'a str, count: Option<u8> },
}
