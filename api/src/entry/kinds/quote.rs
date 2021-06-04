use super::*;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryQuote<'a> {
    #[serde(flatten)]
    pub base: EntryBaseData<'a>,
    pub entries: Entries<'a>,
    pub by: Option<&'a str>,
    pub from: Option<&'a str>,
    /// If the automatically-inserted quotation marks should be skipped.
    #[serde(default)]
    pub skip_marks: Option<bool>,
}

impl<'a> From<EntryQuote<'a>> for EntryKind<'a> {
    fn from(value: EntryQuote<'a>) -> Self {
        EntryKind::Quote(value)
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_quote() {
        let json = r#"{
  "type": "quote",
  "name": "Running the Game",
  "entries": [
    "Rules enable you and your players to have fun at the table. The rules serve you, not vice versa...",
    "There are the rules of the game, and there are table rules for how the game is played."
  ],
  "by": "Korfel Nailo",
  "from": "The Book of Many Names",
  "skipMarks": true
}"#;

        let object: Entry = EntryQuote {
            base: base(Some("Running the Game")),
            entries: vec![
                "Rules enable you and your players to have fun at the table. The rules serve you, not vice versa...".into(),
                "There are the rules of the game, and there are table rules for how the game is played.".into(),
            ],
            by: Some("Korfel Nailo"),
            from: Some("The Book of Many Names"),
            skip_marks: Some(true),
        }.into();

        check_serde(json, object);
    }
}
