use super::*;

#[allow(dead_code)]
pub type LanguageProficiencies = Vec<LanguageProficiency>;

#[skip_serializing_none]
#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageProficiency {
    pub choose: Option<LanguageProficiencyChoose>,
    pub any: Option<u8>,
    pub any_standard: Option<u8>,
    #[serde(flatten, with = "super::bitflags_as_map")]
    pub languages: BitFlags<Language>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageProficiencyChoose {
    #[serde(with = "super::bitflags_as_seq")]
    pub from: BitFlags<Language>,
    pub count: u8,
}

#[bitflags]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Abyssal,
    Celestial,
    #[serde(rename = "deep speech")]
    DeepSpeech,
    Draconic,
    Infernal,
    Primordial,
    Sylvan,
    Undercommon,
    Dwarvish,
    Elvish,
    Giant,
    Goblin,
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn serde_language_proficiencies() {
        let json = r#"[
  {
    "choose": {
      "from": [
        "sylvan",
        "elvish"
      ],
      "count": 1
    },
    "any": 3,
    "abyssal": true,
    "deep speech": true,
    "draconic": true,
    "giant": true
  },
  {
    "infernal": true,
    "primordial": true
  },
  {
    "anyStandard": 1
  }
]"#;

        let object = vec![
            LanguageProficiency {
                choose: Some(LanguageProficiencyChoose {
                    from: Language::Sylvan | Language::Elvish,
                    count: 1,
                }),
                any: Some(3),
                any_standard: None,
                languages: Language::Abyssal
                    | Language::DeepSpeech
                    | Language::Draconic
                    | Language::Giant,
            },
            LanguageProficiency {
                choose: None,
                any: None,
                any_standard: None,
                languages: Language::Infernal | Language::Primordial,
            },
            LanguageProficiency {
                choose: None,
                any: None,
                any_standard: Some(1),
                languages: BitFlags::empty(),
            },
        ];

        check_serde(json, object);
    }
}
