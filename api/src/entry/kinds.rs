pub(crate) mod ability;
pub(crate) mod actions;
pub(crate) mod attack;
pub(crate) mod bonus;
pub(crate) mod data;
pub(crate) mod dice;
pub(crate) mod entries;
pub(crate) mod flow;
pub(crate) mod homebrew;
pub(crate) mod hr;
pub(crate) mod image;
pub(crate) mod ingredient;
pub(crate) mod inline;
pub(crate) mod inset;
pub(crate) mod item;
pub(crate) mod link;
pub(crate) mod list;
pub(crate) mod opt_feature;
pub(crate) mod options;
pub(crate) mod quote;
pub(crate) mod refs;
pub(crate) mod section;
pub(crate) mod spellcasting;
pub(crate) mod table;
pub(crate) mod variant;

pub use ability::*;
pub use actions::*;
pub use attack::*;
pub use bonus::*;
pub use data::*;
pub use dice::*;
pub use entries::*;
pub use flow::*;
pub use homebrew::*;
pub use hr::*;
pub use image::*;
pub use ingredient::*;
pub use inline::*;
pub use inset::*;
pub use item::*;
pub use link::*;
pub use list::*;
pub use opt_feature::*;
pub use options::*;
pub use quote::*;
pub use refs::*;
pub use section::*;
pub use spellcasting::*;
pub use table::*;
pub use variant::*;

use super::*;

#[cfg(test)]
pub(crate) mod test_utils {
    use super::super::EntryBaseData;
    pub use crate::test_utils::*;

    pub fn base(name: Option<&str>) -> EntryBaseData<'_> {
        EntryBaseData {
            name,
            source: None,
            data: None,
            page: None,
            id: None,
        }
    }
}
