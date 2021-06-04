use super::*;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaBlock<'a> {
    #[serde(borrow)]
    pub dependencies: Option<MetaBlockDependencies<'a>>,
    pub other_sources: Option<MetaBlockOtherSources<'a>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaBlockDependencies<'a> {
    #[serde(borrow)]
    pub monster: Option<Vec<&'a str>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaBlockOtherSources<'a> {
    /// Keys are other sources to be loaded; values are `otherSources` sources from that source to search for.
    #[serde(borrow)]
    pub monster: Option<HashMap<&'a str, &'a str>>,
}
