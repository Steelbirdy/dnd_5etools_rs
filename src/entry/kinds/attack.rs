use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryAttack<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBase<'a>,
    pub attack_type: EntryAttackType,
    pub attack_entries: Entries<'a>,
    pub hit_entries: Entries<'a>,
}

impl<'a> From<EntryAttack<'a>> for Entry<'a> {
    fn from(value: EntryAttack<'a>) -> Self {
        Entry::Entry(EntryKind::Attack(value))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum EntryAttackType {
    MW,
    RW,
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_attack() {
        let json = r#"{
  "type": "attack",
  "name": "Claws",
  "attackType": "MW",
  "attackEntries": [
    "+7 to hit, one target"
  ],
  "hitEntries": [
    "10d4 radiant damage"
  ]
}"#;

        let object: Entry = EntryAttack {
            base: base(Some("Claws")),
            attack_type: EntryAttackType::MW,
            attack_entries: vec!["+7 to hit, one target".into()],
            hit_entries: vec!["10d4 radiant damage".into()],
        }.into();

        check_serde(json, object);
    }
}
