use crate::entry::EntryLinkHref;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryLinkExternal<'a> {
    pub url: &'a str,
}

impl<'a> From<EntryLinkExternal<'a>> for super::EntryLinkHref<'a> {
    fn from(value: EntryLinkExternal<'a>) -> Self {
        EntryLinkHref::External(value)
    }
}
