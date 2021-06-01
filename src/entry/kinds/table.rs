use super::*;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryTable<'a> {
    #[serde(flatten)]
    pub base: EntryBase<'a>,
    pub caption: Option<&'a str>,
    /// Primarily for homebrew use.
    pub intro: Option<Entries<'a>>,
    /// Primarily for homebrew use.
    pub outro: Option<Entries<'a>>,
    #[serde(default)]
    pub is_striped: Option<bool>,
    #[serde(default)]
    pub is_name_generator: Option<bool>,
    pub style: Option<&'a str>,
    pub col_labels: Option<Vec<&'a str>>,
    pub col_styles: Option<Vec<&'a str>>,
    pub row_labels: Option<Vec<&'a str>>,
    pub row_styles: Option<Vec<&'a str>>,
    pub rows: Vec<EntryTableRowKind<'a>>,
    pub footnotes: Option<Entries<'a>>,
}

/// Used to group related tables together; has no effect on rendering.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryTableGroup<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBase<'a>,
    pub tables: Option<Entries<'a>>,
}

impl<'a> From<Vec<EntryTable<'a>>> for EntryTableGroup<'a> {
    fn from(tables: Vec<EntryTable<'a>>) -> Self {
        let tables = tables.into_iter().map(Entry::from).collect();

        Self {
            base: Default::default(),
            tables: Some(tables),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EntryTableRowKind<'a> {
    #[serde(borrow)]
    Entries(Entries<'a>),
    __Row(Box<Entry<'a>>),
}

impl<'a> EntryTableRowKind<'a> {
    #[allow(non_snake_case, dead_code)]
    fn Row(row: EntryTableRow<'a>) -> Self {
        Self::__Row(Box::new(row.into()))
    }
}

impl<'a> From<Entries<'a>> for EntryTableRowKind<'a> {
    fn from(value: Entries<'a>) -> Self {
        Self::Entries(value)
    }
}

impl<'a> From<EntryTableRow<'a>> for EntryTableRowKind<'a> {
    fn from(value: EntryTableRow<'a>) -> Self {
        Self::Row(value)
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryTableRow<'a> {
    #[serde(flatten)]
    pub base: EntryBase<'a>,
    pub style: Option<&'a str>,
    pub row: Entries<'a>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryTableCell<'a> {
    #[serde(borrow, flatten)]
    pub base: EntryBase<'a>,
    pub width: Option<i64>,
    pub roll: EntryTableCellRoll,
    pub entry: Option<Box<Entry<'a>>>,
}

impl<'a> From<EntryTable<'a>> for EntryKind<'a> {
    fn from(value: EntryTable<'a>) -> Self {
        EntryKind::Table(value)
    }
}

impl<'a> From<EntryTableGroup<'a>> for EntryKind<'a> {
    fn from(value: EntryTableGroup<'a>) -> Self {
        EntryKind::TableGroup(value)
    }
}

impl<'a> From<EntryTableRow<'a>> for EntryKind<'a> {
    fn from(value: EntryTableRow<'a>) -> Self {
        EntryKind::TableRow(value)
    }
}

impl<'a> From<EntryTableCell<'a>> for EntryKind<'a> {
    fn from(value: EntryTableCell<'a>) -> Self {
        EntryKind::TableCell(value)
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EntryTableCellRoll {
    Range {
        min: i64,
        max: i64,
        pad: Option<bool>,
    },
    Exact {
        exact: i64,
        pad: Option<bool>,
    },
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_table() {
        let json = r#"{
  "type": "table",
  "caption": "Demons Summoned",
  "isStriped": true,
  "colLabels": [
    "d6",
    "Demons Summoned"
  ],
  "rows": [
    [
      {
        "type": "cell",
        "width": 3,
        "roll": {
          "min": 3,
          "max": 4
        }
      },
      "Two demons of challenge rating 1 or lower"
    ],
    [
      "3-4",
      "Four demons of challenge rating 1/2 or lower"
    ],
    {
      "type": "row",
      "row": [
        "5-6",
        "Eight demons of challenge rating 1/4 or lower"
      ]
    }
  ]
}"#;

        let object: Entry = EntryTable {
            base: Default::default(),
            caption: Some("Demons Summoned"),
            intro: None,
            outro: None,
            is_striped: Some(true),
            is_name_generator: None,
            style: None,
            col_labels: Some(vec!["d6", "Demons Summoned"]),
            col_styles: None,
            row_labels: None,
            row_styles: None,
            rows: vec![
                vec![
                    EntryTableCell {
                        base: Default::default(),
                        width: Some(3),
                        roll: EntryTableCellRoll::Range {
                            min: 3,
                            max: 4,
                            pad: None,
                        },
                        entry: None,
                    }
                    .into(),
                    "Two demons of challenge rating 1 or lower".into(),
                ]
                .into(),
                vec![
                    "3-4".into(),
                    "Four demons of challenge rating 1/2 or lower".into(),
                ]
                .into(),
                EntryTableRow {
                    base: Default::default(),
                    style: None,
                    row: vec![
                        "5-6".into(),
                        "Eight demons of challenge rating 1/4 or lower".into(),
                    ],
                }
                .into(),
            ],
            footnotes: None,
        }
        .into();

        check_serde(json, object);
    }
}
