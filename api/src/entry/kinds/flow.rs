use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryFlowchart<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBaseData<'a>,
    // The schema actually doesn't specify that they have to be flow blocks,
    // but it _does_ specify that they have to be actual entry structs, not strings.
    pub blocks: Vec<EntryKind<'a>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryFlowBlock<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBaseData<'a>,
    pub entries: Option<Entries<'a>>,
}

impl<'a> From<EntryFlowchart<'a>> for EntryKind<'a> {
    fn from(value: EntryFlowchart<'a>) -> Self {
        EntryKind::Flowchart(value)
    }
}

impl<'a> From<EntryFlowBlock<'a>> for EntryKind<'a> {
    fn from(value: EntryFlowBlock<'a>) -> Self {
        EntryKind::FlowBlock(value)
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_flowchart() {
        let json = r#"{
  "type": "flowchart",
  "blocks": [
    {
      "type": "flowBlock",
      "name": "Chapter 1: The Unicorn and the Hags",
      "entries": [
        "Chapter 1 summary"
      ]
    },
    {
      "type": "flowBlock",
      "name": "Chapter 2: An Invitation Extended",
      "entries": [
        "Chapter 2 summary"
      ]
    }
  ]
}"#;

        let object: Entry = EntryFlowchart {
            base: Default::default(),
            blocks: vec![
                EntryKind::FlowBlock(EntryFlowBlock {
                    base: base(Some("Chapter 1: The Unicorn and the Hags")),
                    entries: Some(vec!["Chapter 1 summary".into()]),
                }),
                EntryKind::FlowBlock(EntryFlowBlock {
                    base: base(Some("Chapter 2: An Invitation Extended")),
                    entries: Some(vec!["Chapter 2 summary".into()]),
                }),
            ],
        }
        .into();

        check_serde(json, object);
    }
}
