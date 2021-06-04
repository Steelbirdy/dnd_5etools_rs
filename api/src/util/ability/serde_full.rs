use super::*;

pub(crate) mod bitflags_as_seq {
    use super::*;
    use enumflags2::BitFlags;
    use enumflags2::_internal::RawBitFlags;
    use serde::{ser::SerializeSeq, Serialize, Serializer};

    pub use crate::serde_utils::bitflags_as_seq::deserialize;

    #[allow(dead_code)]
    pub fn serialize<S>(flags: &BitFlags<Ability>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Full(#[serde(serialize_with = "Ability::serialize_full")] Ability);

        let mut flags = flags.iter().map(Full).collect::<Vec<_>>();
        flags.sort_by_key(|f| <Ability as RawBitFlags>::bits(f.0));

        let mut seq = s.serialize_seq(Some(flags.len()))?;

        for flag in flags {
            seq.serialize_element(&flag)?;
        }

        seq.end()
    }
}

pub(crate) mod bitflags_as_map {
    use super::*;
    use enumflags2::BitFlags;
    use enumflags2::_internal::RawBitFlags;
    use serde::{ser::SerializeMap, Serialize, Serializer};

    pub use crate::serde_utils::bitflags_as_map::deserialize;

    #[allow(dead_code)]
    pub fn serialize<S>(flags: &BitFlags<Ability>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Full(#[serde(serialize_with = "Ability::serialize_full")] Ability);

        let mut flags = flags.iter().map(Full).collect::<Vec<_>>();
        flags.sort_by_key(|f| <Ability as RawBitFlags>::bits(f.0));

        let mut map = s.serialize_map(Some(flags.len()))?;

        for flag in flags {
            map.serialize_entry(&flag, &true)?;
        }

        map.end()
    }
}
