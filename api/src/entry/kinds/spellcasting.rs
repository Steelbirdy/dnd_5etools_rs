use super::*;
use crate::util::ability::Ability;
use serde::ser::SerializeMap;
use serde::Serializer;
use serde_json::Number;
use std::collections::HashMap;

// For now, this doesn't support the integer keys as given in the API.
// A simple regex replace of r"(\d): \{" with r#""$1": {"# should do.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntrySpellcasting<'a> {
    #[serde(flatten)]
    pub base: EntryBase<'a>,
    pub name: &'a str,
    pub header_entries: Option<Entries<'a>>,
    pub constant: Option<ArrayOfSpell<'a>>,
    pub will: Option<ArrayOfSpell<'a>>,
    pub ritual: Option<ArrayOfSpell<'a>>,
    pub rest: Option<EntrySpellcastingFrequency<'a>>,
    pub daily: Option<EntrySpellcastingFrequency<'a>>,
    pub weekly: Option<EntrySpellcastingFrequency<'a>>,
    pub spells: Option<SpellsByLevel<'a>>,
    /// Allows the above properties to be specified, but not rendered.
    /// Useful if e.g. a creature can only cast one spell, and this is rendered in the header line.
    pub hidden: Option<Vec<EntrySpellcastingProperty>>,
    pub footer_entries: Option<Entries<'a>>,
    pub ability: Option<Ability>,
    /// Defaults to ["trait"]
    #[serde(default)]
    pub display_as: EntrySpellcastingDisplayAs,
}

pub type ArrayOfSpell<'a> = Vec<Spell<'a>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Spell<'a> {
    Text(&'a str),
    Entry { entry: &'a str, hidden: bool },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntrySpellcastingFrequency<'a>(#[serde(borrow)] HashMap<&'a str, ArrayOfSpell<'a>>);

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SpellsByLevel<'a>(#[serde(borrow)] HashMap<u8, EntrySpellcastingLevels<'a>>);

impl<'a> Serialize for SpellsByLevel<'a> {
    fn serialize<S>(&self, s: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut entries = self.0.iter().collect::<Vec<_>>();
        entries.sort_by_key(|(k, _)| *k);

        let mut map = s.serialize_map(Some(entries.len()))?;

        for (k, v) in entries {
            map.serialize_entry(k, v)?;
        }

        map.end()
    }
}

// In the 5e.tools API, this is known as entrySpellcasting_level1to9.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntrySpellcastingLevels<'a> {
    pub lower: Option<Number>,
    pub slots: Option<Number>,
    #[serde(borrow)]
    pub spells: Vec<&'a str>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntrySpellcastingProperty {
    Constant,
    Will,
    Rest,
    Daily,
    Weekly,
    Ritual,
    Spells,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntrySpellcastingDisplayAs {
    Trait,
    Action,
}

impl Default for EntrySpellcastingDisplayAs {
    fn default() -> Self {
        EntrySpellcastingDisplayAs::Trait
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn ser_spells_by_level() {
        let json = r#"{
  "0": {
    "spells": [
      "fire bolt",
      "true strike"
    ]
  },
  "1": {
    "spells": [
      "chromatic orb",
      "magic missile"
    ]
  },
  "3": {
    "lower": 3,
    "slots": 2,
    "spells": [
      "fireball"
    ]
  }
}"#;

        let object = SpellsByLevel(HashMap::from_iter(vec![
            (
                0,
                EntrySpellcastingLevels {
                    lower: None,
                    slots: None,
                    spells: vec!["fire bolt", "true strike"],
                },
            ),
            (
                1,
                EntrySpellcastingLevels {
                    lower: None,
                    slots: None,
                    spells: vec!["chromatic orb", "magic missile"],
                },
            ),
            (
                3,
                EntrySpellcastingLevels {
                    lower: Some(Number::from(3u8)),
                    slots: Some(Number::from(2u8)),
                    spells: vec!["fireball"],
                },
            ),
        ]));

        check_ser(&object, Ok(json));
    }

    #[test]
    fn de_spells_by_level() {
        let json1 = r#"{
  "0": {
    "spells": [
      "fire bolt",
      "true strike"
    ]
  },
  "1": {
    "spells": [
      "chromatic orb",
      "magic missile"
    ]
  },
  "3": {
    "lower": 3,
    "slots": 2,
    "spells": [
      "fireball"
    ]
  }
}"#;
        let object = SpellsByLevel(HashMap::from_iter(vec![
            (
                0,
                EntrySpellcastingLevels {
                    lower: None,
                    slots: None,
                    spells: vec!["fire bolt", "true strike"],
                },
            ),
            (
                1,
                EntrySpellcastingLevels {
                    lower: None,
                    slots: None,
                    spells: vec!["chromatic orb", "magic missile"],
                },
            ),
            (
                3,
                EntrySpellcastingLevels {
                    lower: Some(Number::from(3u8)),
                    slots: Some(Number::from(2u8)),
                    spells: vec!["fireball"],
                },
            ),
        ]));

        check_de(json1, Ok(object));
    }

    #[test]
    fn serde_entry_spellcasting() {
        let json = r#"{
  "name": "Innate Spellcasting",
  "headerEntries": [
    "The unicorn's innate spellcasting ability is Charisma (spell save {@dc 14})."
  ],
  "will": [
    "{@spell detect evil and good}",
    "{@spell druidcraft}",
    "{@spell pass without trace}"
  ],
  "daily": {
    "1e": [
      "{@spell calm emotions}",
      "{@spell dispel evil and good}",
      "{@spell entangle}"
    ]
  },
  "ability": "cha",
  "displayAs": "trait"
}"#;

        let object = EntrySpellcasting {
            base: Default::default(),
            name: "Innate Spellcasting",
            header_entries: Some(vec![
                "The unicorn's innate spellcasting ability is Charisma (spell save {@dc 14})."
                    .into(),
            ]),
            constant: None,
            will: Some(vec![
                Spell::Text("{@spell detect evil and good}"),
                Spell::Text("{@spell druidcraft}"),
                Spell::Text("{@spell pass without trace}"),
            ]),
            ritual: None,
            rest: None,
            daily: Some(EntrySpellcastingFrequency(HashMap::from_iter(vec![(
                "1e",
                vec![
                    Spell::Text("{@spell calm emotions}"),
                    Spell::Text("{@spell dispel evil and good}"),
                    Spell::Text("{@spell entangle}"),
                ],
            )]))),
            weekly: None,
            spells: None,
            hidden: None,
            footer_entries: None,
            ability: Some(Ability::Charisma),
            display_as: Default::default(),
        };

        check_serde(json, object);
    }
}
