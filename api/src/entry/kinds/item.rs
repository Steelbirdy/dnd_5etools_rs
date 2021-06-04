use super::*;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryItem<'a> {
    #[serde(flatten)]
    pub base: EntryBase<'a>,
    pub style: Option<&'a str>,
    pub name: &'a str,
    #[serde(flatten)]
    pub kind: EntryItemKind<'a>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryItemSub<'a> {
    #[serde(flatten)]
    pub base: EntryBase<'a>,
    pub name: &'a str,
    pub entry: Box<Entry<'a>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryItemSpell<'a> {
    #[serde(flatten)]
    pub base: EntryBase<'a>,
    pub name: &'a str,
    pub entry: Box<Entry<'a>>,
}

impl<'a> From<EntryItem<'a>> for EntryKind<'a> {
    fn from(value: EntryItem<'a>) -> Self {
        EntryKind::Item(value)
    }
}

impl<'a> From<EntryItemSub<'a>> for EntryKind<'a> {
    fn from(value: EntryItemSub<'a>) -> Self {
        EntryKind::ItemSub(value)
    }
}

impl<'a> From<EntryItemSpell<'a>> for EntryKind<'a> {
    fn from(value: EntryItemSpell<'a>) -> Self {
        EntryKind::ItemSpell(value)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryItemKind<'a> {
    #[serde(borrow)]
    Entry(Box<Entry<'a>>),
    Entries(Entries<'a>),
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_item() {
        let json = r#"{
  "type": "item",
  "name": "A similar story here",
  "entry": "Blah blah blah"
}"#;

        let object: Entry = EntryItem {
            base: base(None),
            style: None,
            name: "A similar story here",
            kind: EntryItemKind::Entry(Box::new("Blah blah blah".into())),
        }
        .into();

        check_serde(json, object);
    }

    #[test]
    fn serde_entry_item_sub() {
        let json = r#"{
  "type": "itemSub",
  "name": "A sub-item used in some places.",
  "entry": "For example, XGE's complex traps"
}"#;

        let object: Entry = EntryItemSub {
            base: base(None),
            name: "A sub-item used in some places.",
            entry: Box::new("For example, XGE's complex traps".into()),
        }
        .into();

        check_serde(json, object);
    }
}
