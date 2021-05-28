use super::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryDice<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBase<'a>,
    pub to_roll: Option<Vec<EntryDiceToRoll>>,
    pub rollable: Option<bool>,
}

impl<'a> From<EntryDice<'a>> for Entry<'a> {
    fn from(value: EntryDice<'a>) -> Self {
        Entry::Entry(EntryKind::Dice(value))
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryDiceToRoll {
    pub number: i64,
    pub faces: i64,
    /// (Warning: unused)
    pub modifier: Option<i64>,
    /// (Warning: unused)
    pub hide_modifier: Option<bool>,
}
