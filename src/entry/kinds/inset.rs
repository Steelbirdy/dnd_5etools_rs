use super::*;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryInset<'a> {
    #[serde(flatten)]
    pub base: EntryBase<'a>,
    pub entries: Entries<'a>,
    pub style: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryInsetReadaloud<'a> {
    #[serde(flatten)]
    pub base: EntryBase<'a>,
    pub entries: Entries<'a>,
    pub style: Option<&'a str>,
}

impl<'a> From<EntryInset<'a>> for Entry<'a> {
    fn from(value: EntryInset<'a>) -> Self {
        Entry::Entry(EntryKind::Inset(value))
    }
}

impl<'a> From<EntryInsetReadaloud<'a>> for Entry<'a> {
    fn from(value: EntryInsetReadaloud<'a>) -> Self {
        Entry::Entry(EntryKind::InsetReadaloud(value))
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_inset() {
        let json = r#"{
  "type": "inset",
  "name": "Inset Title (Optional)",
  "entries": [
    "This is a text inset/insert/sidebar/etc."
  ]
}"#;

        let object: Entry = EntryInset {
            base: base(Some("Inset Title (Optional)")),
            entries: vec!["This is a text inset/insert/sidebar/etc.".into()],
            style: None,
        }
        .into();

        check_serde(json, object);
    }

    #[test]
    fn serde_entry_inset_readaloud() {
        let json = r#"{
  "type": "insetReadaloud",
  "name": "Same As Above",
  "entries": [
    "But a different color"
  ]
}"#;

        let object: Entry = EntryInsetReadaloud {
            base: base(Some("Same As Above")),
            entries: vec!["But a different color".into()],
            style: None,
        }
        .into();

        check_serde(json, object);
    }
}
