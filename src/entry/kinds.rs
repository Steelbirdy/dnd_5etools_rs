pub mod bonus;
pub mod dice;
pub mod entries;
pub mod homebrew;
pub mod inline;
pub mod list;
pub mod options;
pub mod quote;
pub mod section;
pub mod table;

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
