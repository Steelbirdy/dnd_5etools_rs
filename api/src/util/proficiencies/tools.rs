use super::*;

#[allow(dead_code)]
pub type ToolProficiencies = Vec<ToolProficiency>;

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolProficiency {
    #[serde(flatten, with = "super::bitflags_as_map")]
    pub tools: BitFlags<Tool>,
    pub any: Option<u8>,
    pub choose: Option<ToolProficiencyChoose>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolProficiencyChoose {
    #[serde(with = "super::bitflags_as_seq")]
    pub from: BitFlags<Tool>,
    pub count: u8,
}

#[bitflags]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tool {
    #[serde(rename = "artisan's tools")]
    ArtisansTools,
    #[serde(rename = "cartographer's tools")]
    CartographersTools,
    #[serde(rename = "disguise kit")]
    DisguiseKit,
    #[serde(rename = "forgery kit")]
    ForgeryKit,
    #[serde(rename = "gaming set")]
    GamingSet,
    #[serde(rename = "herbalism kit")]
    HerbalismKit,
    #[serde(rename = "musical instrument")]
    MusicalInstrument,
    #[serde(rename = "navigator's tools")]
    NavigatorsTools,
    #[serde(rename = "thieves' tools")]
    ThievesTools,
    #[serde(rename = "vehicles (land)")]
    VehiclesLand,
    #[serde(rename = "vehicles (water)")]
    VehiclesWater,
    #[serde(rename = "alchemist's supplies")]
    AlchemistsSupplies,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn serde_tool_proficiencies() {
        let json = r#"[
  {
    "forgery kit": true,
    "alchemist's supplies": true,
    "any": 3,
    "choose": {
      "from": [
        "artisan's tools",
        "gaming set",
        "thieves' tools"
      ],
      "count": 2
    }
  },
  {
    "herbalism kit": true,
    "vehicles (land)": true
  },
  {
    "choose": {
      "from": [
        "vehicles (land)",
        "vehicles (water)"
      ],
      "count": 1
    }
  }
]"#;

        let object = vec![
            ToolProficiency {
                tools: Tool::ForgeryKit | Tool::AlchemistsSupplies,
                any: Some(3),
                choose: Some(ToolProficiencyChoose {
                    from: Tool::ArtisansTools | Tool::ThievesTools | Tool::GamingSet,
                    count: 2,
                }),
            },
            ToolProficiency {
                tools: Tool::HerbalismKit | Tool::VehiclesLand,
                any: None,
                choose: None,
            },
            ToolProficiency {
                tools: BitFlags::empty(),
                any: None,
                choose: Some(ToolProficiencyChoose {
                    from: Tool::VehiclesLand | Tool::VehiclesWater,
                    count: 1,
                }),
            },
        ];

        check_serde(json, object);
    }
}
