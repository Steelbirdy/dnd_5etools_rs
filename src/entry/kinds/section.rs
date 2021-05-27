use super::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntrySection<'a> {
    #[serde(flatten, borrow)]
    pub base: EntryBase<'a>,
    pub alias: Option<Vec<&'a str>>,
    pub entries: Entries<'a>,
}

impl<'a> From<EntrySection<'a>> for Entry<'a> {
    fn from(value: EntrySection<'a>) -> Self {
        Entry::Entry(EntryKind::Section(value))
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_section() {
        let json = r#"{
  "type": "section",
  "name": "Running the Game",
  "entries": [
    "Rules enable you and your players to have fun at the table. The rules serve you, not vice versa...",
    "There are the rules of the game, and there are table rules for how the game is played."
  ]
}"#;

        let object = Entry::Entry(EntryKind::Section(EntrySection {
            base: base(Some("Running the Game")),
            alias: None,
            entries: vec![
                Entry::String("Rules enable you and your players to have fun at the table. The rules serve you, not vice versa..."),
                Entry::String("There are the rules of the game, and there are table rules for how the game is played."),
            ],
        }));

        check_serde(json, object);
    }
}
