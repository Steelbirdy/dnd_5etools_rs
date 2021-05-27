use super::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryOptions<'a> {
    #[serde(flatten)]
    pub base: EntryBase<'a>,
    /// Used to specify how many of the listed options can be chosen as e.g. permanent character features.
    /// Leave blank for transient choices.
    pub count: Option<u8>,
    pub style: Option<&'a str>,
    pub entries: Entries<'a>,
}

impl<'a> From<EntryOptions<'a>> for Entry<'a> {
    fn from(value: EntryOptions<'a>) -> Self {
        Entry::Entry(EntryKind::Options(value))
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_options() {
        let json = r#"{
  "type": "options",
  "name": "Example Options",
  "count": 3,
  "style": "example-style",
  "entries": [
    "Hello",
    "World"
  ]
}"#;

        let object = Entry::Entry(EntryKind::Options(EntryOptions {
            base: base(Some("Example Options")),
            count: Some(3),
            style: Some("example-style"),
            entries: vec![Entry::String("Hello"), Entry::String("World")],
        }));

        check_serde(json, object);
    }
}
