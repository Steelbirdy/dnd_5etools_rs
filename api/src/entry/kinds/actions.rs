use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryActions<'a> {
    #[serde(flatten)]
    pub base: EntryBase<'a>,
    pub name: &'a str,
    pub entries: Entries<'a>,
}

impl<'a> From<EntryActions<'a>> for EntryKind<'a> {
    fn from(value: EntryActions<'a>) -> Self {
        EntryKind::Actions(value)
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_actions() {
        let json = r#"{
  "type": "actions",
  "name": "Claw",
  "entries": [
    "Description of the claw attack"
  ]
}"#;

        let object: Entry = EntryActions {
            base: Default::default(),
            name: "Claw",
            entries: vec!["Description of the claw attack".into()],
        }
        .into();

        check_serde(json, object);
    }
}
