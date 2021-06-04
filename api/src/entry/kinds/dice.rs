use super::*;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryDice<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBase<'a>,
    pub to_roll: Option<Vec<EntryDiceToRoll>>,
    pub rollable: Option<bool>,
}

impl<'a> From<EntryDice<'a>> for EntryKind<'a> {
    fn from(value: EntryDice<'a>) -> Self {
        EntryKind::Dice(value)
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryDiceToRoll {
    pub number: u16,
    pub faces: u16,
    /// (Warning: unused)
    pub modifier: Option<i16>,
    /// (Warning: unused)
    pub hide_modifier: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_dice() {
        let json = r#"{
  "type": "dice",
  "toRoll": [
    {
      "number": 1,
      "faces": 4
    },
    {
      "number": 2,
      "faces": 7,
      "modifier": 0
    },
    {
      "number": 3,
      "faces": 10,
      "modifier": 0,
      "hideModifier": true
    }
  ],
  "rollable": true
}"#;

        let object: Entry = EntryDice {
            base: Default::default(),
            to_roll: Some(vec![
                EntryDiceToRoll {
                    number: 1,
                    faces: 4,
                    modifier: None,
                    hide_modifier: None,
                }
                .into(),
                EntryDiceToRoll {
                    number: 2,
                    faces: 7,
                    modifier: Some(0),
                    hide_modifier: None,
                }
                .into(),
                EntryDiceToRoll {
                    number: 3,
                    faces: 10,
                    modifier: Some(0),
                    hide_modifier: Some(true),
                }
                .into(),
            ]),
            rollable: Some(true),
        }
        .into();

        check_serde(json, object);
    }
}
