mod default;

pub use default::DefaultStringRenderer;

use super::{
    tags::{Tag, TagName},
    Result,
};

pub trait StringRenderer {
    fn render(&mut self, input: &str) -> Result<String>;

    fn render_tag(&mut self, tag: Tag) -> Result<String> {
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

    fn render_bold(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_italic(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_strikethrough(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_underline(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_note(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_attack(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_h(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_color(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_highlight(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_help(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_comic(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_comic_h1(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_comic_h2(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_comic_h3(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_comic_h4(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_comic_note(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_dc(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_dice(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_damage(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_d20(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_hit(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_chance(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_recharge(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_hit_your_spell_attack(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_scale_dice(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_scale_damage(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_filter(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_link(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_5etools(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_footnote(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_homebrew(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_skill(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_sense(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_area(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_loader(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_book(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_adventure(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_deity(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_class_feature(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_subclass_feature(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_spell(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_item(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_class(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_creature(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_condition_disease_status(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_background(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_race(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_optional_feature(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_reward(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_feat(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_psionic(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_object(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_cult_boon(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_trap_hazard(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_variant_rule(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_table(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_vehicle(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_vehicle_upgrade(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_action(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_language(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_char_option(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
    fn render_recipe(&mut self, _args: Vec<&str>) -> Result<String> {
        unimplemented!()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RenderErrorKind {
    ArgCount,
    ArgFormat(&'static str),
}
