use super::*;
use crate::util::ability::Ability;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryAbilityDc<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBase<'a>,
    pub name: &'a str,
    pub attributes: Vec<EntryAbilityAttribute>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryAbilityAttackMod<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBase<'a>,
    pub name: &'a str,
    pub attributes: Vec<EntryAbilityAttribute>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryAbilityGeneric<'a> {
    #[serde(flatten)]
    pub base: EntryBase<'a>,
    pub text: &'a str,
    pub attributes: Option<Vec<EntryAbilityAttribute>>,
}

impl<'a> From<EntryAbilityDc<'a>> for EntryKind<'a> {
    fn from(value: EntryAbilityDc<'a>) -> Self {
        EntryKind::AbilityDc(value)
    }
}

impl<'a> From<EntryAbilityAttackMod<'a>> for EntryKind<'a> {
    fn from(value: EntryAbilityAttackMod<'a>) -> Self {
        EntryKind::AbilityAttackMod(value)
    }
}

impl<'a> From<EntryAbilityGeneric<'a>> for EntryKind<'a> {
    fn from(value: EntryAbilityGeneric<'a>) -> Self {
        EntryKind::AbilityGeneric(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum EntryAbilityAttribute {
    #[serde(rename = "str")]
    Strength,
    #[serde(rename = "dex")]
    Dexterity,
    #[serde(rename = "con")]
    Constitution,
    #[serde(rename = "int")]
    Intelligence,
    #[serde(rename = "wis")]
    Wisdom,
    #[serde(rename = "cha")]
    Charisma,
    #[serde(rename = "spellcasting")]
    Spellcasting,
}

impl From<Ability> for EntryAbilityAttribute {
    fn from(ab: Ability) -> Self {
        use Ability::*;

        match ab {
            Strength => Self::Strength,
            Dexterity => Self::Dexterity,
            Constitution => Self::Constitution,
            Intelligence => Self::Intelligence,
            Wisdom => Self::Wisdom,
            Charisma => Self::Charisma,
        }
    }
}

impl TryFrom<EntryAbilityAttribute> for Ability {
    type Error = ();

    fn try_from(value: EntryAbilityAttribute) -> std::result::Result<Self, Self::Error> {
        use EntryAbilityAttribute::*;

        let ret = match value {
            Strength => Self::Strength,
            Dexterity => Self::Dexterity,
            Constitution => Self::Constitution,
            Intelligence => Self::Intelligence,
            Wisdom => Self::Wisdom,
            Charisma => Self::Charisma,
            Spellcasting => return Err(()),
        };

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_ability_dc() {
        let json = r#"{
  "type": "abilityDc",
  "name": "Spell Save DC",
  "attributes": [
    "spellcasting",
    "int"
  ]
}"#;

        let object: Entry = EntryAbilityDc {
            base: Default::default(),
            name: "Spell Save DC",
            attributes: vec![
                EntryAbilityAttribute::Spellcasting,
                EntryAbilityAttribute::Intelligence,
            ],
        }
        .into();

        check_serde(json, object);
    }

    #[test]
    fn serde_entry_ability_attack_mod() {
        let json = r#"{
  "type": "abilityAttackMod",
  "name": "Spell Attack Modifier",
  "attributes": [
    "int",
    "cha",
    "dex"
  ]
}"#;

        let object: Entry = EntryAbilityAttackMod {
            base: Default::default(),
            name: "Spell Attack Modifier",
            attributes: vec![
                EntryAbilityAttribute::Intelligence,
                EntryAbilityAttribute::Charisma,
                EntryAbilityAttribute::Dexterity,
            ],
        }
        .into();

        check_serde(json, object);
    }

    #[test]
    fn serde_entry_ability_generic() {
        let json = r#"{
  "type": "abilityGeneric",
  "name": "Artificer Infusion DC",
  "text": "8 + your intelligence modifier"
}"#;

        let object: Entry = EntryAbilityGeneric {
            base: base(Some("Artificer Infusion DC")),
            text: "8 + your intelligence modifier",
            attributes: None,
        }
        .into();

        check_serde(json, object);
    }
}
