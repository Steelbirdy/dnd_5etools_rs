use crate::entry::Entries;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[allow(dead_code)]
pub type Prerequisite<'a> = Vec<PrerequisiteItem<'a>>;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrerequisiteItem<'a> {
    pub level: Option<PrerequisiteLevel<'a>>,
    pub pact: Option<PrerequisitePact>,
    pub patron: Option<PrerequisitePatron>,
    pub spell: Option<Entries<'a>>,
    pub feature: Option<Entries<'a>>,
    pub item: Option<Entries<'a>>,
    /// A free text prerequisite
    pub other: Option<&'a str>,
    /// A free text prerequisite, with a shortened form for list display.
    pub other_summary: Option<PrerequisiteOtherSummary<'a>>,
    pub race: Option<Vec<PrerequisiteRace<'a>>>,
    pub ability: Option<Vec<PrerequisiteAbility>>,
    pub spellcasting: Option<bool>,
    /// Renders with the updated text found in UA2020: Feats
    pub spellcasting2020: Option<bool>,
    pub psionics: Option<bool>,
    pub proficiency: Option<Vec<PrerequisiteProficiency>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrerequisiteLevel<'a> {
    Simple(u8),
    Custom {
        level: u8,
        #[serde(borrow)]
        class: Option<PrerequisiteLevelClass<'a>>,
        subclass: Option<PrerequisiteLevelSubclass<'a>>,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PrerequisiteLevelClass<'a>(#[serde(borrow)] pub PrerequisiteLevelItem<'a>);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PrerequisiteLevelSubclass<'a>(#[serde(borrow)] pub PrerequisiteLevelItem<'a>);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PrerequisiteLevelItem<'a> {
    pub name: &'a str,
    pub source: Option<&'a str>,
    /// Governs whether or not the class name is visible in the list display/prerequisite line.
    /// *Not* recommended for features which implicitly carry a class restriction, such as Eldritch Invocations.
    pub visible: Option<bool>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum PrerequisitePact {
    Chain,
    Tome,
    Blade,
    Talisman,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum PrerequisitePatron {
    #[serde(rename = "The Archfey")]
    Archfey,
    #[serde(rename = "The Fiend")]
    Fiend,
    #[serde(rename = "The Great Old One")]
    GreatOldOne,
    #[serde(rename = "The Hexblade")]
    Hexblade,
    #[serde(rename = "The Raven Queen")]
    RavenQueen,
    #[serde(rename = "The Seeker")]
    Seeker,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrerequisiteOtherSummary<'a> {
    pub entry: &'a str,
    /// Used in short/list displays
    pub entry_summary: &'a str,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrerequisiteRace<'a> {
    pub name: &'a str,
    pub display_entry: Option<&'a str>,
    pub subrace: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrerequisiteAbility {
    pub str: Option<i64>,
    pub dex: Option<i64>,
    pub con: Option<i64>,
    pub int: Option<i64>,
    pub wis: Option<i64>,
    pub cha: Option<i64>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PrerequisiteProficiency {
    pub armor: Option<PrerequisiteArmorProficiency>,
    pub weapon: Option<PrerequisiteWeaponProficiency>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrerequisiteArmorProficiency {
    Light,
    Medium,
    Heavy,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrerequisiteWeaponProficiency {
    Simple,
    Martial,
}
