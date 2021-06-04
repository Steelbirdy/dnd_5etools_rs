use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[bitflags]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DamageType {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}

pub type DamageImmunityArray<'a> = Option<Vec<DamageImmunity<'a>>>;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DamageImmunity<'a> {
    Simple(DamageType),
    Special {
        special: &'a str,
    },
    Annotated {
        #[serde(rename = "preNote")]
        pre_note: Option<&'a str>,
        immune: DamageImmunityArray<'a>,
        note: Option<&'a str>,
        cond: Option<bool>,
    },
}

pub type DamageResistArray<'a> = Option<Vec<DamageResistance<'a>>>;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DamageResistance<'a> {
    Simple(DamageType),
    Special {
        special: &'a str,
    },
    Annotated {
        #[serde(rename = "preNote")]
        pre_note: Option<&'a str>,
        resist: DamageResistArray<'a>,
        note: Option<&'a str>,
        cond: Option<bool>,
    },
}

pub type DamageVulnerabilityArray<'a> = Option<Vec<DamageVulnerability<'a>>>;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DamageVulnerability<'a> {
    Simple(DamageType),
    Special {
        special: &'a str,
    },
    Annotated {
        #[serde(rename = "preNote")]
        pre_note: Option<&'a str>,
        vulnerable: DamageVulnerabilityArray<'a>,
        note: Option<&'a str>,
        cond: Option<bool>,
    },
}
