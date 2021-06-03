use super::*;

#[allow(dead_code)]
pub type ArmorProficiencies = Vec<ArmorProficiency>;

#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArmorProficiency(#[serde(with = "super::bitflags_as_map")] BitFlags<Armor>);

#[bitflags]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Armor {
    Light,
    Medium,
    Heavy,
    #[serde(rename = "shield|phb")]
    Shield,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn serde_armor_proficiencies() {
        let json = r#"[
  {
    "light": true,
    "heavy": true
  },
  {
    "medium": true,
    "shield|phb": true
  }
]"#;

        let object = vec![
            ArmorProficiency(Armor::Light | Armor::Heavy),
            ArmorProficiency(Armor::Medium | Armor::Shield),
        ];

        check_serde(json, object);
    }
}
