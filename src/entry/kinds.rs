pub mod ability;
pub mod actions;
pub mod attack;
pub mod bonus;
pub mod dice;
pub mod entries;
pub mod flow;
pub mod homebrew;
pub mod image;
pub mod ingredient;
pub mod inline;
pub mod inset;
pub mod item;
pub mod link;
pub mod list;
pub mod opt_feature;
pub mod options;
pub mod quote;
pub mod section;
pub mod table;
pub mod variant;

use super::*;

#[cfg(test)]
mod test_utils {
    use super::super::EntryBase;
    pub use crate::test_utils::*;

    pub fn base(name: Option<&str>) -> EntryBase<'_> {
        EntryBase {
            name,
            source: None,
            data: None,
            page: None,
            id: None,
        }
    }
}
