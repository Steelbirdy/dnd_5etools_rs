use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryHr;

impl From<EntryHr> for Entry<'_> {
    fn from(value: EntryHr) -> Self {
        Entry::Entry(EntryKind::Hr(value))
    }
}
