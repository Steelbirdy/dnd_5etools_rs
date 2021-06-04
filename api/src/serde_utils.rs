pub mod bitflags_as_seq {
    use enumflags2::_internal::RawBitFlags;
    use enumflags2::{BitFlag, BitFlags};
    use serde::{
        de::{SeqAccess, Visitor},
        ser::SerializeSeq,
        Deserialize, Deserializer, Serialize, Serializer,
    };
    use std::fmt;
    use std::marker::PhantomData;

    pub fn serialize<S, P>(flags: &BitFlags<P>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        P: Serialize + BitFlag,
        <P as RawBitFlags>::Numeric: Ord,
    {
        let mut flags = flags.iter().collect::<Vec<_>>();
        flags.sort_by_key(|f| f.bits());

        let mut seq = s.serialize_seq(Some(flags.len()))?;

        for flag in flags {
            seq.serialize_element(&flag)?;
        }

        seq.end()
    }

    pub fn deserialize<'de, D, P>(d: D) -> Result<BitFlags<P>, D::Error>
    where
        D: Deserializer<'de>,
        P: Deserialize<'de> + BitFlag,
    {
        struct ProficiencyArrayVisitor<T>(PhantomData<T>);

        impl<'v, T> Visitor<'v> for ProficiencyArrayVisitor<T>
        where
            T: Deserialize<'v> + BitFlag,
        {
            type Value = BitFlags<T>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("an array of proficiencies")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'v>,
            {
                let mut flags = BitFlags::empty();
                while let Some(flag) = seq.next_element::<T>()? {
                    flags.insert(flag);
                }
                Ok(flags)
            }
        }

        d.deserialize_seq(ProficiencyArrayVisitor(PhantomData::<P>))
    }
}

pub mod bitflags_as_map {
    use enumflags2::_internal::RawBitFlags;
    use enumflags2::{BitFlag, BitFlags};
    use serde::{
        de::{MapAccess, Visitor},
        ser::SerializeMap,
        Deserialize, Deserializer, Serialize, Serializer,
    };
    use std::fmt;
    use std::marker::PhantomData;

    pub fn serialize<S, P>(flags: &BitFlags<P>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        P: Serialize + BitFlag,
        <P as RawBitFlags>::Numeric: Ord,
    {
        let mut flags = flags.iter().collect::<Vec<_>>();
        flags.sort_by_key(|f| f.bits());

        let mut map = s.serialize_map(Some(flags.len()))?;

        for flag in flags {
            map.serialize_entry(&flag, &true)?;
        }

        map.end()
    }

    pub fn deserialize<'de, D, P>(d: D) -> Result<BitFlags<P>, D::Error>
    where
        D: Deserializer<'de>,
        P: Deserialize<'de> + BitFlag,
    {
        struct ProficiencyMapVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for ProficiencyMapVisitor<T>
        where
            T: Deserialize<'de> + BitFlag,
        {
            type Value = BitFlags<T>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a map of proficiency to bool")
            }

            fn visit_map<A>(
                self,
                mut access: A,
            ) -> Result<Self::Value, <A as MapAccess<'de>>::Error>
            where
                A: MapAccess<'de>,
            {
                let mut bitflags = BitFlags::empty();

                while let Some((key, value)) = access.next_entry::<T, bool>()? {
                    if value {
                        bitflags.insert(key);
                    }
                }

                Ok(bitflags)
            }
        }

        d.deserialize_map(ProficiencyMapVisitor(PhantomData::<P>))
    }
}
