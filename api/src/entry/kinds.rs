pub mod ability;
pub mod actions;
pub mod attack;
pub mod bonus;
pub mod data;
pub mod dice;
pub mod entries;
pub mod flow;
pub mod homebrew;
pub mod hr;
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
pub mod refs;
pub mod section;
pub mod spellcasting;
pub mod table;
pub mod variant;

pub mod all {
    pub use super::ability::*;
    pub use super::actions::*;
    pub use super::attack::*;
    pub use super::bonus::*;
    pub use super::data::*;
    pub use super::dice::*;
    pub use super::entries::*;
    pub use super::flow::*;
    pub use super::homebrew::*;
    pub use super::hr::*;
    pub use super::image::*;
    pub use super::ingredient::*;
    pub use super::inline::*;
    pub use super::inset::*;
    pub use super::item::*;
    pub use super::link::*;
    pub use super::list::*;
    pub use super::opt_feature::*;
    pub use super::options::*;
    pub use super::quote::*;
    pub use super::refs::*;
    pub use super::section::*;
    pub use super::spellcasting::*;
    pub use super::table::*;
    pub use super::variant::*;
}

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
