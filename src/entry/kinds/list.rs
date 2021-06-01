use super::*;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryList<'a> {
    #[serde(flatten)]
    pub base: EntryBase<'a>,
    /// Number of columns the content should be split into.
    /// Note that the full value is only displayed on wide screens,
    /// and screens below certain widths will see an appropriately reduced number of columns.
    pub columns: Option<i64>,
    pub style: Option<&'a str>,
    pub items: Entries<'a>,
}

impl<'a> From<EntryList<'a>> for EntryKind<'a> {
    fn from(value: EntryList<'a>) -> Self {
        EntryKind::List(value)
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_list() {
        let json = r#"{
  "type": "list",
  "columns": 1,
  "style": "list-nohang",
  "items": [
    "Hello, world!",
    "Never gonna give you up"
  ]
}"#;

        let object: Entry = EntryList {
            base: Default::default(),
            columns: Some(1),
            style: Some("list-nohang"),
            items: vec!["Hello, world!".into(), "Never gonna give you up".into()],
        }
        .into();

        check_serde(json, object);
    }
}
