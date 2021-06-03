mod serde_abbrev;
mod serde_full;

use enumflags2::bitflags;
use serde::{ser::Serializer, Deserialize, Serialize};

#[bitflags]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ability {
    #[serde(rename = "str", alias = "strength")]
    Strength,
    #[serde(rename = "dex", alias = "dexterity")]
    Dexterity,
    #[serde(rename = "con", alias = "constitution")]
    Constitution,
    #[serde(rename = "int", alias = "intelligence")]
    Intelligence,
    #[serde(rename = "wis", alias = "wisdom")]
    Wisdom,
    #[serde(rename = "cha", alias = "charisma")]
    Charisma,
}

#[allow(dead_code)]
impl Ability {
    pub fn name_full(&self) -> &'static str {
        use Ability::*;

        match self {
            Strength => "strength",
            Dexterity => "dexterity",
            Constitution => "constitution",
            Intelligence => "intelligence",
            Wisdom => "wisdom",
            Charisma => "charisma",
        }
    }

    pub fn name_abbrev(&self) -> &'static str {
        &self.name_full()[..3]
    }

    /// The default behavior
    pub fn serialize_abbrev<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.serialize(s)
    }

    pub fn serialize_full<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_unit_variant("Ability", *self as u32, self.name_full())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Ability::Strength     => String::from(r#""strength""#))]
    #[test_case(Ability::Dexterity    => String::from(r#""dexterity""#))]
    #[test_case(Ability::Constitution => String::from(r#""constitution""#))]
    #[test_case(Ability::Intelligence => String::from(r#""intelligence""#))]
    #[test_case(Ability::Wisdom       => String::from(r#""wisdom""#))]
    #[test_case(Ability::Charisma     => String::from(r#""charisma""#))]
    fn serialize_ability_full(ability: Ability) -> String {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Full(#[serde(serialize_with = "Ability::serialize_full")] Ability);

        serde_json::to_string_pretty(&Full(ability)).unwrap()
    }

    #[test_case(Ability::Strength     => String::from(r#""str""#))]
    #[test_case(Ability::Dexterity    => String::from(r#""dex""#))]
    #[test_case(Ability::Constitution => String::from(r#""con""#))]
    #[test_case(Ability::Intelligence => String::from(r#""int""#))]
    #[test_case(Ability::Wisdom       => String::from(r#""wis""#))]
    #[test_case(Ability::Charisma     => String::from(r#""cha""#))]
    fn serialize_ability(ability: Ability) -> String {
        serde_json::to_string_pretty(&ability).unwrap()
    }

    #[test_case(r#""strength""#     => Ability::Strength)]
    #[test_case(r#""str""#          => Ability::Strength)]
    #[test_case(r#""dexterity""#    => Ability::Dexterity)]
    #[test_case(r#""dex""#          => Ability::Dexterity)]
    #[test_case(r#""constitution""# => Ability::Constitution)]
    #[test_case(r#""con""#          => Ability::Constitution)]
    #[test_case(r#""intelligence""# => Ability::Intelligence)]
    #[test_case(r#""int""#          => Ability::Intelligence)]
    #[test_case(r#""wisdom""#       => Ability::Wisdom)]
    #[test_case(r#""wis""#          => Ability::Wisdom)]
    #[test_case(r#""charisma""#     => Ability::Charisma)]
    #[test_case(r#""cha""#          => Ability::Charisma)]
    fn deserialize_ability(input: &str) -> Ability {
        serde_json::from_str(input).unwrap()
    }
}
