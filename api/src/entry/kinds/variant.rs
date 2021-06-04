use super::*;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryVariant<'a> {
    #[serde(flatten)]
    pub base: EntryBaseData<'a>,
    pub name: &'a str,
    pub entries: Entries<'a>,
    pub variant_source: Option<EntryVariantSource<'a>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryVariantInner<'a> {
    #[serde(flatten)]
    pub base: EntryBaseData<'a>,
    pub name: &'a str,
    pub entries: Entries<'a>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryVariantSub<'a> {
    #[serde(flatten)]
    pub base: EntryBaseData<'a>,
    pub name: &'a str,
    pub entries: Entries<'a>,
}

impl<'a> From<EntryVariant<'a>> for EntryKind<'a> {
    fn from(value: EntryVariant<'a>) -> Self {
        EntryKind::Variant(value)
    }
}

impl<'a> From<EntryVariantInner<'a>> for EntryKind<'a> {
    fn from(value: EntryVariantInner<'a>) -> Self {
        EntryKind::VariantInner(value)
    }
}

impl<'a> From<EntryVariantSub<'a>> for EntryKind<'a> {
    fn from(value: EntryVariantSub<'a>) -> Self {
        EntryKind::VariantSub(value)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryVariantSource<'a> {
    pub source: &'a str,
    pub page: i64,
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;
    use crate::entry::kinds::entries::EntryEntries;

    #[test]
    fn serde_entry_variant() {
        let json = r#"{
  "type": "variant",
  "name": "Better Monster",
  "entries": [
    "Some variant monster text",
    {
      "type": "entries",
      "name": "Inline Header",
      "entries": [
        "Text text text",
        {
          "type": "variantSub",
          "name": "These can have child items",
          "entries": [
            "And the header style for them is unique, so this exists to cope with that."
          ]
        }
      ]
    }
  ]
}"#;

        let object: Entry = EntryVariant {
            base: Default::default(),
            name: "Better Monster",
            entries: vec![
                "Some variant monster text".into(),
                EntryEntries {
                    base: base(Some("Inline Header")),
                    alias: None,
                    entries: vec![
                        "Text text text".into(),
                        EntryVariantSub {
                            base: Default::default(),
                            name: "These can have child items",
                            entries: vec![
                                "And the header style for them is unique, so this exists to cope with that.".into(),
                            ],
                        }.into(),
                    ],
                }.into(),
            ],
            variant_source: None
        }.into();

        check_serde(json, object);
    }
}
