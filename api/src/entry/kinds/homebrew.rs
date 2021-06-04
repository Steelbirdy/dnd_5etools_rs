use super::*;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryHomebrew<'a> {
    #[serde(flatten, borrow)]
    pub base: EntryBaseData<'a>,
    pub entries: Option<Entries<'a>>,
    pub moved_to: Option<Box<Entry<'a>>>,
    pub old_entries: Option<Entries<'a>>,
}

impl<'a> From<EntryHomebrew<'a>> for EntryKind<'a> {
    fn from(value: EntryHomebrew<'a>) -> Self {
        EntryKind::Homebrew(value)
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_homebrew() {
        let json = r#"{
  "type": "homebrew",
  "name": "Running the Game",
  "entries": [
    "Rules enable you and your players to have fun at the table. The rules serve you, not vice versa...",
    "There are the rules of the game, and there are table rules for how the game is played."
  ],
  "movedTo": "Nowhere",
  "oldEntries": [
    "Never gonna give you up",
    "Never gonna let you down"
  ]
}"#;

        let object: Entry = EntryHomebrew {
            base: base(Some("Running the Game")),
            entries: Some(vec![
                "Rules enable you and your players to have fun at the table. The rules serve you, not vice versa...".into(),
                "There are the rules of the game, and there are table rules for how the game is played.".into(),
            ]),
            moved_to: Some(Box::new("Nowhere".into())),
            old_entries: Some(vec![
                "Never gonna give you up".into(),
                "Never gonna let you down".into(),
            ]),
        }.into();

        check_serde(json, object);
    }
}
