use super::*;

#[allow(dead_code)]
pub type SkillProficiencies<'a> = Vec<SkillProficiency<'a>>;

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillProficiency<'a> {
    #[serde(flatten, with = "super::bitflags_as_map")]
    pub skills: BitFlags<Skill>,
    pub tool: Option<bool>,
    #[serde(borrow)]
    pub choose: Option<SkillProficiencyChoose<'a>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillProficiencyChoose<'a> {
    #[serde(borrow)]
    pub from: Vec<SkillProficiencyChooseItem<'a>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SkillProficiencyChooseItem<'a> {
    Skill(Skill),
    Tools {
        #[serde(borrow)]
        tool: Vec<&'a str>,
    },
    Tool {
        tool: bool,
    },
}

#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Skill {
    Athletics,
    Acrobatics,
    #[serde(rename = "sleight of hand")]
    SleightOfHand,
    Stealth,
    Arcana,
    History,
    Investigation,
    Nature,
    Religion,
    #[serde(rename = "animal handling")]
    AnimalHandling,
    Insight,
    Medicine,
    Perception,
    Survival,
    Deception,
    Intimidation,
    Performance,
    Persuasion,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn serde_skill_proficiencies() {
        let json = r#"[
  {
    "nature": true,
    "survival": true,
    "choose": {
      "from": [
        "religion",
        "perception",
        {
          "tool": [
            "thieves' tools",
            "herbalism kit"
          ]
        },
        "animal handling"
      ]
    }
  },
  {
    "athletics": true,
    "arcana": true,
    "tool": true
  },
  {
    "choose": {
      "from": [
        "history",
        "investigation",
        {
          "tool": false
        }
      ]
    }
  }
]"#;

        let object = vec![
            SkillProficiency {
                skills: Skill::Nature | Skill::Survival,
                tool: None,
                choose: Some(SkillProficiencyChoose {
                    from: vec![
                        SkillProficiencyChooseItem::Skill(Skill::Religion),
                        SkillProficiencyChooseItem::Skill(Skill::Perception),
                        SkillProficiencyChooseItem::Tools {
                            tool: vec!["thieves' tools", "herbalism kit"],
                        },
                        SkillProficiencyChooseItem::Skill(Skill::AnimalHandling),
                    ],
                }),
            },
            SkillProficiency {
                skills: Skill::Athletics | Skill::Arcana,
                tool: Some(true),
                choose: None,
            },
            SkillProficiency {
                skills: BitFlags::empty(),
                tool: None,
                choose: Some(SkillProficiencyChoose {
                    from: vec![
                        SkillProficiencyChooseItem::Skill(Skill::History),
                        SkillProficiencyChooseItem::Skill(Skill::Investigation),
                        SkillProficiencyChooseItem::Tool { tool: false },
                    ],
                }),
            },
        ];

        check_serde(json, object);
    }
}
