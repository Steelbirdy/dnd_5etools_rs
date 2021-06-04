mod default;
pub mod utils;

pub use default::DefaultStringRenderer;

use super::{
    tags::{Tag, TagName},
    Result,
};
use std::fmt::Display;
use std::ops::{Bound, RangeBounds};
use thiserror::Error;

pub trait RenderString {
    fn render(&self, input: &str) -> Result<String>;

    /// Dispatches to the correct rendering function.
    /// [RenderString] implementations should not override the default behavior.
    fn render_tag(&self, tag: Tag) -> Result<String> {
        match tag.name {
            TagName::Bold => self.render_bold(tag.args),
            TagName::Italic => self.render_italic(tag.args),
            TagName::Strikethrough => self.render_strikethrough(tag.args),
            TagName::Underline => self.render_underline(tag.args),
            TagName::Note => self.render_note(tag.args),
            TagName::Attack => self.render_attack(tag.args),
            TagName::H => self.render_h(tag.args),
            TagName::Color => self.render_color(tag.args),
            TagName::Highlight => self.render_highlight(tag.args),
            TagName::Help => self.render_help(tag.args),
            TagName::Comic => self.render_comic(tag.args),
            TagName::ComicH1 => self.render_comic_h1(tag.args),
            TagName::ComicH2 => self.render_comic_h2(tag.args),
            TagName::ComicH3 => self.render_comic_h3(tag.args),
            TagName::ComicH4 => self.render_comic_h4(tag.args),
            TagName::ComicNote => self.render_comic_note(tag.args),
            TagName::Dc => self.render_dc(tag.args),
            TagName::Dice => self.render_dice(tag.args),
            TagName::Damage => self.render_damage(tag.args),
            TagName::D20 => self.render_d20(tag.args),
            TagName::Hit => self.render_hit(tag.args),
            TagName::Chance => self.render_chance(tag.args),
            TagName::Recharge => self.render_recharge(tag.args),
            TagName::HitYourSpellAttack => self.render_hit_your_spell_attack(tag.args),
            TagName::ScaleDice => self.render_scale_dice(tag.args),
            TagName::ScaleDamage => self.render_scale_damage(tag.args),
            TagName::Filter => self.render_filter(tag.args),
            TagName::Link => self.render_link(tag.args),
            TagName::_5eTools => self.render_5etools(tag.args),
            TagName::Footnote => self.render_footnote(tag.args),
            TagName::Homebrew => self.render_homebrew(tag.args),
            TagName::Skill => self.render_skill(tag.args),
            TagName::Sense => self.render_sense(tag.args),
            TagName::Area => self.render_area(tag.args),
            TagName::Loader => self.render_loader(tag.args),
            TagName::Book => self.render_book(tag.args),
            TagName::Adventure => self.render_adventure(tag.args),
            TagName::Deity => self.render_deity(tag.args),
            TagName::ClassFeature => self.render_class_feature(tag.args),
            TagName::SubclassFeature => self.render_subclass_feature(tag.args),
            TagName::Spell => self.render_spell(tag.args),
            TagName::Item => self.render_item(tag.args),
            TagName::Class => self.render_class(tag.args),
            TagName::Creature => self.render_creature(tag.args),
            TagName::ConditionDiseaseStatus => self.render_condition_disease_status(tag.args),
            TagName::Background => self.render_background(tag.args),
            TagName::Race => self.render_race(tag.args),
            TagName::OptionalFeature => self.render_optional_feature(tag.args),
            TagName::Reward => self.render_reward(tag.args),
            TagName::Feat => self.render_feat(tag.args),
            TagName::Psionic => self.render_psionic(tag.args),
            TagName::Object => self.render_object(tag.args),
            TagName::CultBoon => self.render_cult_boon(tag.args),
            TagName::TrapHazard => self.render_trap_hazard(tag.args),
            TagName::VariantRule => self.render_variant_rule(tag.args),
            TagName::Table => self.render_table(tag.args),
            TagName::Vehicle => self.render_vehicle(tag.args),
            TagName::VehicleUpgrade => self.render_vehicle_upgrade(tag.args),
            TagName::Action => self.render_action(tag.args),
            TagName::Language => self.render_language(tag.args),
            TagName::CharOption => self.render_char_option(tag.args),
            TagName::Recipe => self.render_recipe(tag.args),
        }
    }

    fn render_bold(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_bold").into())
    }
    fn render_italic(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_italic").into())
    }
    fn render_strikethrough(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_strikethrough").into())
    }
    fn render_underline(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_underline").into())
    }
    fn render_note(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_note").into())
    }
    fn render_attack(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_attack").into())
    }
    fn render_h(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_h").into())
    }
    fn render_color(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_color").into())
    }
    fn render_highlight(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_highlight").into())
    }
    fn render_help(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_help").into())
    }
    fn render_comic(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_comic").into())
    }
    fn render_comic_h1(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_comic_h1").into())
    }
    fn render_comic_h2(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_comic_h2").into())
    }
    fn render_comic_h3(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_comic_h3").into())
    }
    fn render_comic_h4(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_comic_h4").into())
    }
    fn render_comic_note(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_comic_note").into())
    }
    fn render_dc(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_dc").into())
    }
    fn render_dice(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_dice").into())
    }
    fn render_damage(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_damage").into())
    }
    fn render_d20(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_d20").into())
    }
    fn render_hit(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_hit").into())
    }
    fn render_chance(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_chance").into())
    }
    fn render_recharge(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_recharge").into())
    }
    fn render_hit_your_spell_attack(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_hit_your_spell_attack").into())
    }
    fn render_scale_dice(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_scale_dice").into())
    }
    fn render_scale_damage(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_scale_damage").into())
    }
    fn render_filter(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_filter").into())
    }
    fn render_link(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_link").into())
    }
    fn render_5etools(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_5etools").into())
    }
    fn render_footnote(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_footnote").into())
    }
    fn render_homebrew(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_homebrew").into())
    }
    fn render_skill(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_skill").into())
    }
    fn render_sense(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_sense").into())
    }
    fn render_area(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_area").into())
    }
    fn render_loader(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_loader").into())
    }
    fn render_book(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_book").into())
    }
    fn render_adventure(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_adventure").into())
    }
    fn render_deity(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_deity").into())
    }
    fn render_class_feature(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_class_feature").into())
    }
    fn render_subclass_feature(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_subclass_feature").into())
    }
    fn render_spell(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_spell").into())
    }
    fn render_item(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_item").into())
    }
    fn render_class(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_class").into())
    }
    fn render_creature(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_creature").into())
    }
    fn render_condition_disease_status(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_condition_disease_status").into())
    }
    fn render_background(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_background").into())
    }
    fn render_race(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_race").into())
    }
    fn render_optional_feature(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_optional_feature").into())
    }
    fn render_reward(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_reward").into())
    }
    fn render_feat(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_feat").into())
    }
    fn render_psionic(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_psionic").into())
    }
    fn render_object(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_object").into())
    }
    fn render_cult_boon(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_cult_boon").into())
    }
    fn render_trap_hazard(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_trap_hazard").into())
    }
    fn render_variant_rule(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_variant_rule").into())
    }
    fn render_table(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_table").into())
    }
    fn render_vehicle(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_vehicle").into())
    }
    fn render_vehicle_upgrade(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_vehicle_upgrade").into())
    }
    fn render_action(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_action").into())
    }
    fn render_language(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_language").into())
    }
    fn render_char_option(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_char_option").into())
    }
    fn render_recipe(&self, args: Vec<&str>) -> Result<String> {
        let _ = args;
        Err(RenderError::not_implemented("render_recipe").into())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum RenderError {
    #[error("wrong number of arguments (expected {}, found {found})", format_range(.expected))]
    ArgCount {
        expected: (Bound<usize>, Bound<usize>),
        found: usize,
    },
    #[error("invalid argument format: {0}")]
    ArgFormat(String),
    #[error("the function `{0}` has not been implemented")]
    NotImplemented(&'static str),
    #[error("{0}")]
    Custom(String),
}

impl RenderError {
    pub fn arg_count<R>(expected: R, found: usize) -> Self
    where
        R: RangeBounds<usize>,
    {
        Self::ArgCount {
            expected: bounds_from_range(expected),
            found,
        }
    }

    pub fn arg_format<D: Display>(msg: D) -> Self {
        Self::ArgFormat(msg.to_string())
    }

    pub fn not_implemented(fn_name: &'static str) -> Self {
        Self::NotImplemented(fn_name)
    }

    pub fn custom<D: Display>(msg: D) -> Self {
        Self::Custom(msg.to_string())
    }
}

fn format_range((lower, upper): &(Bound<usize>, Bound<usize>)) -> String {
    let lower_bound = match lower {
        Bound::Unbounded => 0usize,
        Bound::Excluded(n) => *n + 1,
        Bound::Included(n) => *n,
    };

    match upper {
        Bound::Unbounded => {
            format!("{} or more", lower_bound)
        }
        Bound::Excluded(n) => {
            let n = if *n == 0 { 0 } else { n - 1 };

            format!("between {} and {}", lower_bound, n)
        }
        Bound::Included(n) => {
            format!("between {} and {}", lower_bound, n)
        }
    }
}

pub(crate) fn bounds_from_range<R>(range: R) -> (Bound<usize>, Bound<usize>)
where
    R: RangeBounds<usize>,
{
    use Bound::*;

    let (lower, upper) = (range.start_bound(), range.end_bound());

    let lower = match lower {
        Unbounded => Unbounded,
        Included(n) => Included(*n),
        Excluded(n) => Included(*n + 1),
    };

    let upper = match upper {
        Unbounded => Unbounded,
        Included(n) => Included(*n),
        Excluded(n) => {
            let n = if *n == 0 { 0 } else { *n - 1 };
            Included(n)
        }
    };

    (lower, upper)
}
