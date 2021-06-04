mod external;
mod internal;

pub use external::*;
pub use internal::*;

use super::*;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryLink<'a> {
    #[serde(flatten)]
    pub base: EntryBase<'a>,
    pub text: &'a str,
    pub href: EntryLinkHref<'a>,
}

impl<'a> From<EntryLink<'a>> for EntryKind<'a> {
    fn from(value: EntryLink<'a>) -> Self {
        EntryKind::Link(value)
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum EntryLinkHref<'a> {
    #[serde(borrow)]
    Internal(internal::EntryLinkInternal<'a>),
    External(external::EntryLinkExternal<'a>),
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn serde_entry_link_internal() {
        let json = r#"{
  "type": "link",
  "text": "such as a link to the homepage",
  "href": {
    "type": "internal",
    "path": "5etools.html"
  }
}"#;

        let object: Entry = EntryLink {
            base: base(None),
            text: "such as a link to the homepage",
            href: EntryLinkInternal {
                path: "5etools.html",
                hash: None,
                hash_pre_encoded: None,
                subhashes: None,
                hover: None,
            }
            .into(),
        }
        .into();

        check_serde(json, object);
    }

    #[test]
    fn serde_entry_link_external() {
        let json = r#"{
  "type": "link",
  "text": "The full 'entry' schema.",
  "href": {
    "type": "external",
    "url": "https://raw.githubusercontent.com/TheGiddyLimit/TheGiddyLimit.github.io/master/test/schema/entry.json"
  }
}"#;

        let object: Entry = EntryLink {
            base: base(None),
            text: "The full 'entry' schema.",
            href: EntryLinkExternal {
                url: "https://raw.githubusercontent.com/TheGiddyLimit/TheGiddyLimit.github.io/master/test/schema/entry.json",
            }.into(),
        }.into();

        check_serde(json, object);
    }
}
