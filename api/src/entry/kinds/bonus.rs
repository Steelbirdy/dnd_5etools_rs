use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryBonus<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBase<'a>,
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryBonusSpeed<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBase<'a>,
    pub value: i64,
}

impl<'a> From<EntryBonus<'a>> for EntryKind<'a> {
    fn from(value: EntryBonus<'a>) -> Self {
        EntryKind::Bonus(value)
    }
}

impl<'a> From<EntryBonusSpeed<'a>> for EntryKind<'a> {
    fn from(value: EntryBonusSpeed<'a>) -> Self {
        EntryKind::BonusSpeed(value)
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_bonus() {
        let json = r#"{
  "type": "bonus",
  "value": 5
}"#;

        let object: Entry = EntryBonus {
            base: Default::default(),
            value: 5,
        }
        .into();

        check_serde(json, object);
    }

    #[test]
    fn serde_entry_bonus_speed() {
        let json = r#"{
  "type": "bonusSpeed",
  "value": 5
}"#;

        let object: Entry = EntryBonusSpeed {
            base: Default::default(),
            value: 5,
        }
        .into();

        check_serde(json, object);
    }
}
