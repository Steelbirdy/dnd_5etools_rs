use crate::entry::{kinds::image::EntryImage, Entries};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericFluffArrayItemData<'a> {
    pub name: Option<&'a str>,
    pub source: Option<&'a str>,
    pub images: Option<Vec<EntryImage<'a>>>,
    pub entries: Option<Entries<'a>>,
}

#[allow(dead_code)]
pub type GenericFluffArray<'a> = Vec<GenericFluffArrayItem<'a>>;

// TODO: Maybe figure out a better way to emulate the $$merge schema pre-processor tag?
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenericFluffArrayItem<'a> {
    Sourced {
        name: &'a str,
        source: &'a str,
        images: Option<Vec<EntryImage<'a>>>,
        entries: Option<Entries<'a>>,
    },
    Copied {
        #[serde(flatten)]
        data: GenericFluffArrayItemData<'a>,
        _copy: super::copy::CopyBlock<'a>,
    },
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffObject<'a> {
    #[serde(borrow)]
    pub entries: Option<Entries<'a>>,
    pub images: Option<Vec<EntryImage<'a>>>,
}
