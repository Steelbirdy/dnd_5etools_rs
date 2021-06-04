mod default;
mod utils;

pub use default::DefaultStringRenderer;

use super::{
    tags::{Tag, TagName},
    Result,
};
use std::fmt::Display;
use std::ops::{Bound, RangeBounds};
use thiserror::Error;

pub trait StringRenderer {
    fn render(&self, input: &str) -> Result<String>;

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

    fn render_bold(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_italic(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_strikethrough(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_underline(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_note(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_attack(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_h(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_color(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_highlight(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_help(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_comic(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_comic_h1(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_comic_h2(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_comic_h3(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_comic_h4(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_comic_note(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_dc(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_dice(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_damage(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_d20(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_hit(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_chance(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_recharge(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_hit_your_spell_attack(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_scale_dice(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_scale_damage(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_filter(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_link(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_5etools(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_footnote(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_homebrew(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_skill(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_sense(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_area(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_loader(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_book(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_adventure(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_deity(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_class_feature(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_subclass_feature(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_spell(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_item(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_class(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_creature(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_condition_disease_status(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_background(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_race(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_optional_feature(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_reward(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_feat(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_psionic(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_object(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_cult_boon(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_trap_hazard(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_variant_rule(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_table(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_vehicle(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_vehicle_upgrade(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_action(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_language(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_char_option(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_recipe(&self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
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
    #[error("{0}")]
    Custom(String),
}

impl RenderError {
    pub fn arg_count<R>(expected: R, found: usize) -> Self
    where
        R: RangeBounds<usize>,
    {
        Self::ArgCount {
            expected: utils::bounds_from_range(expected),
            found,
        }
    }

    pub fn arg_format<D: Display>(msg: D) -> Self {
        Self::ArgFormat(msg.to_string())
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
