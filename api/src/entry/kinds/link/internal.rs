use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryLinkInternal<'a> {
    pub path: &'a str,
    pub hash: Option<&'a str>,
    pub hash_pre_encoded: Option<bool>,
    pub subhashes: Option<Vec<EntryLinkInternalSubhash<'a>>>,
    pub hover: Option<EntryLinkInternalHover<'a>>,
}

impl<'a> From<EntryLinkInternal<'a>> for super::EntryLinkHref<'a> {
    fn from(value: EntryLinkInternal<'a>) -> Self {
        super::EntryLinkHref::Internal(value)
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EntryLinkInternalSubhash<'a> {
    #[serde(rename_all = "camelCase")]
    Value {
        key: &'a str,
        value: &'a str,
        pre_encoded: Option<bool>,
    },
    #[serde(rename_all = "camelCase")]
    Values {
        key: &'a str,
        values: Vec<&'a str>,
        pre_encoded: Option<bool>,
    },
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryLinkInternalHover<'a> {
    pub page: &'a str,
    pub source: &'a str,
    /// Optional; overrides the href hash for hover handlers.
    pub hash: Option<&'a str>,
    pub hash_pre_encoded: Option<bool>,
}
