use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryAbilityDc<'a> {
    pub name: &'a str,
    #[serde(borrow, flatten)]
    pub base: EntryBase<'a>,
    pub attributes: Vec<EntryAbilityAttribute>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryAbilityAttackMod<'a> {
    pub name: &'a str,
    #[serde(borrow, flatten)]
    pub base: EntryBase<'a>,
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

impl<'a> From<EntryAbilityDc<'a>> for Entry<'a> {
    fn from(value: EntryAbilityDc<'a>) -> Self {
        Entry::Entry(EntryKind::AbilityDc(value))
    }
}

impl<'a> From<EntryAbilityAttackMod<'a>> for Entry<'a> {
    fn from(value: EntryAbilityAttackMod<'a>) -> Self {
        Entry::Entry(EntryKind::AbilityAttackMod(value))
    }
}

impl<'a> From<EntryAbilityGeneric<'a>> for Entry<'a> {
    fn from(value: EntryAbilityGeneric<'a>) -> Self {
        Entry::Entry(EntryKind::AbilityGeneric(value))
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

        let object = Entry::Entry(EntryKind::AbilityDc(EntryAbilityDc {
            name: "Spell Save DC",
            base: base(None),
            attributes: vec![
                EntryAbilityAttribute::Spellcasting,
                EntryAbilityAttribute::Intelligence,
            ],
        }));

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

        let object = Entry::Entry(EntryKind::AbilityAttackMod(EntryAbilityAttackMod {
            name: "Spell Attack Modifier",
            base: base(None),
            attributes: vec![
                EntryAbilityAttribute::Intelligence,
                EntryAbilityAttribute::Charisma,
                EntryAbilityAttribute::Dexterity,
            ],
        }));

        check_serde(json, object);
    }

    #[test]
    fn serde_entry_ability_generic() {
        let json = r#"{
  "type": "abilityGeneric",
  "name": "Artificer Infusion DC",
  "text": "8 + your intelligence modifier"
}"#;

        let object = Entry::Entry(EntryKind::AbilityGeneric(EntryAbilityGeneric {
            base: base(Some("Artificer Infusion DC")),
            text: "8 + your intelligence modifier",
            attributes: None,
        }));

        check_serde(json, object);
    }
}
