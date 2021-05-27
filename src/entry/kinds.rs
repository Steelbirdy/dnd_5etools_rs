pub mod entries;
pub mod section;

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
