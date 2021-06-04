use super::{Error, Result};
use crate::string::lexer::LexemeTag;
use std::convert::TryFrom;
use std::str::FromStr;
use thiserror::Error;

pub enum TagName {
    Bold,
    Italic,
    Strikethrough,
    Underline,
    Note,
    Attack,
    H,
    Color,
    Highlight,
    Help,
    Comic,
    ComicH1,
    ComicH2,
    ComicH3,
    ComicH4,
    ComicNote,
    Dc,
    Dice,
    Damage,
    D20,
    Hit,
    Chance,
    Recharge,
    HitYourSpellAttack,
    ScaleDice,
    ScaleDamage,
    Filter,
    Link,
    _5eTools,
    Footnote,
    Homebrew,
    Skill,
    Sense,
    Area,
    Loader,
    Book,
    Adventure,
    Deity,
    ClassFeature,
    SubclassFeature,
    Spell,
    Item,
    Class,
    Creature,
    ConditionDiseaseStatus,
    Background,
    Race,
    OptionalFeature,
    Reward,
    Feat,
    Psionic,
    Object,
    CultBoon,
    TrapHazard,
    VariantRule,
    Table,
    Vehicle,
    VehicleUpgrade,
    Action,
    Language,
    CharOption,
    Recipe,
}
impl FromStr for TagName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "b" | "bold" => Ok(Self::Bold),
            "i" | "italic" => Ok(Self::Italic),
            "s" | "strike" => Ok(Self::Strikethrough),
            "u" | "underline" => Ok(Self::Underline),
            "note" => Ok(Self::Note),
            "atk" => Ok(Self::Attack),
            "h" => Ok(Self::H),
            "color" => Ok(Self::Color),
            "highlight" => Ok(Self::Highlight),
            "help" => Ok(Self::Help),
            "comic" => Ok(Self::Comic),
            "comicH1" => Ok(Self::ComicH1),
            "comicH2" => Ok(Self::ComicH2),
            "comicH3" => Ok(Self::ComicH3),
            "comicH4" => Ok(Self::ComicH4),
            "comicNote" => Ok(Self::ComicNote),
            "dc" => Ok(Self::Dc),
            "dice" => Ok(Self::Dice),
            "damage" => Ok(Self::Damage),
            "d20" => Ok(Self::D20),
            "hit" => Ok(Self::Hit),
            "chance" => Ok(Self::Chance),
            "recharge" => Ok(Self::Recharge),
            "hitYourSpellAttack" => Ok(Self::HitYourSpellAttack),
            "scaledice" => Ok(Self::ScaleDice),
            "scaledamage" => Ok(Self::ScaleDamage),
            "filter" => Ok(Self::Filter),
            "link" => Ok(Self::Link),
            "5etools" => Ok(Self::_5eTools),
            "footnote" => Ok(Self::Footnote),
            "homebrew" => Ok(Self::Homebrew),
            "skill" => Ok(Self::Skill),
            "sense" => Ok(Self::Sense),
            "area" => Ok(Self::Area),
            "loader" => Ok(Self::Loader),
            "book" => Ok(Self::Book),
            "adventure" => Ok(Self::Adventure),
            "deity" => Ok(Self::Deity),
            "classFeature" => Ok(Self::ClassFeature),
            "subclassFeature" => Ok(Self::SubclassFeature),
            "spell" => Ok(Self::Spell),
            "item" => Ok(Self::Item),
            "class" => Ok(Self::Class),
            "creature" => Ok(Self::Creature),
            "condition" | "disease" | "status" => Ok(Self::ConditionDiseaseStatus),
            "background" => Ok(Self::Background),
            "race" => Ok(Self::Race),
            "optfeature" => Ok(Self::OptionalFeature),
            "reward" => Ok(Self::Reward),
            "feat" => Ok(Self::Feat),
            "psionic" => Ok(Self::Psionic),
            "object" => Ok(Self::Object),
            "cult" | "boon" => Ok(Self::CultBoon),
            "trap" | "hazard" => Ok(Self::TrapHazard),
            "variantrule" => Ok(Self::VariantRule),
            "table" => Ok(Self::Table),
            "vehicle" => Ok(Self::Vehicle),
            "vehupgrade" => Ok(Self::VehicleUpgrade),
            "action" => Ok(Self::Action),
            "language" => Ok(Self::Language),
            "charoption" => Ok(Self::CharOption),
            "recipe" => Ok(Self::Recipe),
            _ => Err(TagError::UnrecognizedName(s.to_string()).into()),
        }
    }
}

pub struct Tag<'a> {
    pub name: TagName,
    pub args: Vec<&'a str>,
}

impl<'a> Tag<'a> {
    pub fn new(name: &'a str, args: Vec<&'a str>) -> Result<Self> {
        let name = TagName::from_str(name)?;

        Ok(Self { name, args })
    }
}

impl<'a> TryFrom<LexemeTag<'a>> for Tag<'a> {
    type Error = Error;

    fn try_from(lexeme: LexemeTag<'a>) -> Result<Self> {
        let LexemeTag { name, args } = lexeme;
        let name = TagName::from_str(name)?;

        Ok(Self { name, args })
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum TagError {
    #[error("unrecognized tag name `{0}`")]
    UnrecognizedName(String),
}
