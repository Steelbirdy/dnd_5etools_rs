use super::*;

/// For e.g. Eldritch Invocations which require prerequisite text
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryOptFeature<'a> {
    #[serde(flatten)]
    pub base: EntryBase<'a>,
    pub name: &'a str,
    pub prerequisite: Option<&'a str>,
    // This is missing in the schema, but is included in the render demo.
    pub entries: Option<Entries<'a>>,
}

impl<'a> From<EntryOptFeature<'a>> for Entry<'a> {
    fn from(value: EntryOptFeature<'a>) -> Self {
        Entry::Entry(EntryKind::OptFeature(value))
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_opt_feature() {
        let json = r#"{
  "type": "optfeature",
  "name": "Optional Feature Name",
  "prerequisite": "Optional prerequisite, which can include tags {@spell eldritch blast}",
  "entries": [
    "Optional feature text"
  ]
}"#;

        let object: Entry = EntryOptFeature {
            base: base(None),
            name: "Optional Feature Name",
            prerequisite: Some(
                "Optional prerequisite, which can include tags {@spell eldritch blast}",
            ),
            entries: Some(vec![Entry::String("Optional feature text")]),
        }
        .into();

        check_serde(json, object);
    }
}
