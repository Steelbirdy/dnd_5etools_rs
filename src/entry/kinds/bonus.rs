use super::*;
use serde::{Deserialize, Serialize};

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

impl<'a> From<EntryBonus<'a>> for Entry<'a> {
    fn from(value: EntryBonus<'a>) -> Self {
        Entry::Entry(EntryKind::Bonus(value))
    }
}

impl<'a> From<EntryBonusSpeed<'a>> for Entry<'a> {
    fn from(value: EntryBonusSpeed<'a>) -> Self {
        Entry::Entry(EntryKind::BonusSpeed(value))
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

        let object = Entry::Entry(EntryKind::Bonus(EntryBonus {
            base: base(None),
            value: 5,
        }));

        check_serde(json, object);
    }

    #[test]
    fn serde_entry_bonus_speed() {
        let json = r#"{
  "type": "bonusSpeed",
  "value": 5
}"#;

        let object = Entry::Entry(EntryKind::BonusSpeed(EntryBonusSpeed {
            base: base(None),
            value: 5,
        }));

        check_serde(json, object);
    }
}
