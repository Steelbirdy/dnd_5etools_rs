use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryInline<'a> {
    #[serde(flatten, borrow)]
    pub base: EntryBase<'a>,
    pub entries: Entries<'a>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryInlineBlock<'a> {
    #[serde(flatten, borrow)]
    pub base: EntryBase<'a>,
    pub entries: Entries<'a>,
}

impl<'a> From<EntryInline<'a>> for Entry<'a> {
    fn from(value: EntryInline<'a>) -> Self {
        Entry::Entry(EntryKind::Inline(value))
    }
}

impl<'a> From<EntryInlineBlock<'a>> for Entry<'a> {
    fn from(value: EntryInlineBlock<'a>) -> Self {
        Entry::Entry(EntryKind::InlineBlock(value))
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_inline() {
        let json = r#"{
  "type": "inline",
  "name": "Running the Game",
  "entries": [
    "Rules enable you and your players to have fun at the table. The rules serve you, not vice versa...",
    "There are the rules of the game, and there are table rules for how the game is played."
  ]
}"#;

        let object = Entry::Entry(EntryKind::Inline(EntryInline {
            base: base(Some("Running the Game")),
            entries: vec![
                Entry::String("Rules enable you and your players to have fun at the table. The rules serve you, not vice versa..."),
                Entry::String("There are the rules of the game, and there are table rules for how the game is played."),
            ],
        }));

        check_serde(json, object);
    }

    #[test]
    fn serde_entry_inline_block() {
        let json = r#"{
  "type": "inlineBlock",
  "name": "Running the Game",
  "entries": [
    "Rules enable you and your players to have fun at the table. The rules serve you, not vice versa...",
    "There are the rules of the game, and there are table rules for how the game is played."
  ]
}"#;

        let object = Entry::Entry(EntryKind::InlineBlock(EntryInlineBlock {
            base: base(Some("Running the Game")),
            entries: vec![
                Entry::String("Rules enable you and your players to have fun at the table. The rules serve you, not vice versa..."),
                Entry::String("There are the rules of the game, and there are table rules for how the game is played."),
            ],
        }));

        check_serde(json, object);
    }
}
