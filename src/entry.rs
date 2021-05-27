mod error;
pub mod kinds;
mod render;

use kinds::*;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryBase<'a> {
    pub name: Option<&'a str>,
    pub source: Option<&'a str>,
    /// A generic object for storing special data for external use-cases.
    /// Keys prefixed with \"rd-\" should be added as \"data-\" HTML attributes when rendering to HTML.
    pub data: Option<Value>,
    pub page: Option<i64>,
    pub id: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Entry<'a> {
    Entry(EntryKind<'a>),
    String(&'a str),
    Integer(i64),
}

pub type Entries<'a> = Vec<Entry<'a>>;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum EntryKind<'a> {
    #[serde(borrow)]
    Section(section::EntrySection<'a>),
    Entries(entries::EntryEntries<'a>),
    Homebrew(homebrew::EntryHomebrew<'a>),
    Quote(quote::EntryQuote<'a>),
    Inline(inline::EntryInline<'a>),
    InlineBlock(inline::EntryInlineBlock<'a>),
}
