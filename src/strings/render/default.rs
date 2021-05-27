use super::super::{
    lexer::{Lexeme, Lexer},
    tags::Tag,
    Error, Result,
};
use super::{RenderErrorKind, StringRenderer};
use std::collections::HashSet;
use std::ops::RangeBounds;

pub struct DefaultStringRenderer;

impl DefaultStringRenderer {
    fn render_lexeme(&mut self, lexeme: Lexeme<'_>) -> Result<String> {
        match lexeme {
            Lexeme::Text(text) => self.render_text(text),
            Lexeme::Tag { name, args } => {
                let tag = Tag::new(name, args).map_err(Error::from)?;

                self.render_tag(tag)
            }
            Lexeme::Error(kind) => Err(kind.into()),
        }
    }

    fn render_text(&mut self, text: &str) -> Result<String> {
        Ok(text.to_owned())
    }
}

impl StringRenderer for DefaultStringRenderer {
    fn render(&mut self, input: &str) -> Result<String> {
        Lexer::new(input)
            .map(|lexeme| self.render_lexeme(lexeme))
            .collect::<Result<String>>()
    }

    /// Default behavior is to return the first argument unchanged
    fn render_bold(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_italic(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_strikethrough(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_underline(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_note(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to use [::render_attack_tag]
    fn render_attack(&mut self, args: Vec<&str>) -> Result<String> {
        render_attack_tag(args)
    }

    /// Default behavior is to return ["Hit: "]
    fn render_h(&mut self, _args: Vec<&str>) -> Result<String> {
        Ok("Hit: ".to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_color(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_highlight(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_help(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_comic(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_comic_h1(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_comic_h2(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_comic_h3(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_comic_h4(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_comic_note(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return [format!("DC {}", first_arg)]
    fn render_dc(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        let text = args[0];

        Ok(format!("DC {}", text))
    }

    /// Default behavior is to return the second argument if present, otherwise the first argument
    fn render_dice(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 2 {
            Ok(args[1].to_owned())
        } else if args.len() == 1 {
            Ok(args[0].replace(';', "/"))
        } else {
            unreachable!()
        }
    }

    /// Default behavior is to return the second argument if present, otherwise the first argument
    fn render_damage(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 2 {
            Ok(args[1].to_owned())
        } else if args.len() == 1 {
            Ok(args[0].replace(';', "/"))
        } else {
            unreachable!()
        }
    }

    /// Default behavior is to use [::render_hit_bonus_tag]
    fn render_d20(&mut self, args: Vec<&str>) -> Result<String> {
        render_hit_bonus_tag(args)
    }

    /// Default behavior is to use [::render_hit_bonus_tag]
    fn render_hit(&mut self, args: Vec<&str>) -> Result<String> {
        render_hit_bonus_tag(args)
    }

    /// Default behavior is to return the second argument if present, otherwise [format!("{}%", first_arg)]
    fn render_chance(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 2 {
            Ok(args[1].to_owned())
        } else if args.len() == 1 {
            Ok(format!("{}%", args[0]))
        } else {
            unreachable!()
        }
    }

    /// Default behavior is to use [::render_recharge_tag]
    fn render_recharge(&mut self, args: Vec<&str>) -> Result<String> {
        render_recharge_tag(args)
    }

    /// Default behavior is to return ["your spell attack modifier"]
    fn render_hit_your_spell_attack(&mut self, _args: Vec<&str>) -> Result<String> {
        Ok("your spell attack modifier".to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_scale_dice(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_scale_damage(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_filter(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_link(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_5etools(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_footnote(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to use [::render_homebrew_tag]
    fn render_homebrew(&mut self, args: Vec<&str>) -> Result<String> {
        render_homebrew_tag(args)
    }

    /// Default behavior is to return the first argument unchanged
    fn render_skill(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_sense(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to use [::render_area_tag]
    fn render_area(&mut self, args: Vec<&str>) -> Result<String> {
        render_area_tag(args)
    }

    /// Default behavior is to return the first argument unchanged
    fn render_loader(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_book(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the first argument unchanged
    fn render_adventure(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;
        Ok(args[0].to_owned())
    }

    /// Default behavior is to return the fourth argument if present, otherwise the first argument
    fn render_deity(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 4 {
            Ok(args[3].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the sixth argument if present, otherwise the first argument
    fn render_class_feature(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 6 {
            Ok(args[5].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the eighth argument if present, otherwise the first argument
    fn render_subclass_feature(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 8 {
            Ok(args[7].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_spell(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_item(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_class(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_creature(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_condition_disease_status(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_background(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_race(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_optional_feature(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_reward(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_feat(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_psionic(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_object(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_cult_boon(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_trap_hazard(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_variant_rule(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_table(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_vehicle(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_vehicle_upgrade(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_action(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_language(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_char_option(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }

    /// Default behavior is to return the third argument if present, otherwise the first argument
    fn render_recipe(&mut self, args: Vec<&str>) -> Result<String> {
        check_arg_count(1.., args.len())?;

        if args.len() >= 3 {
            Ok(args[2].to_owned())
        } else {
            Ok(args[0].to_owned())
        }
    }
}

fn check_arg_count<R: RangeBounds<usize>>(expected: R, actual: usize) -> Result<()> {
    if expected.contains(&actual) {
        Ok(())
    } else {
        Err(Error::from(RenderErrorKind::ArgCount))
    }
}

fn render_attack_tag(args: Vec<&str>) -> Result<String> {
    check_arg_count(1..=1, args.len())?;

    fn render_group(group: String) -> String {
        let group = group.chars().collect::<HashSet<_>>();

        static FIRST_GROUP_PARTS: [(char, &str); 4] = [
            ('m', "Melee "),
            ('r', "Ranged "),
            ('g', "Magical "),
            ('a', "Area "),
        ];
        static SECOND_GROUP_PARTS: [(char, &str); 2] = [('w', "Weapon "), ('s', "Spell ")];

        let mut buf = String::new();

        for (part, str) in &FIRST_GROUP_PARTS {
            if group.contains(part) {
                buf.push_str(*str);
                break;
            }
        }
        for (part, str) in &SECOND_GROUP_PARTS {
            if group.contains(part) {
                buf.push_str(*str);
                break;
            }
        }
        buf
    }

    let groups = args[0].to_lowercase();
    let groups = groups
        .split(',')
        .filter_map(|s| {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        })
        .collect::<Vec<_>>();

    let len = groups.len();

    let groups = if len > 1 {
        let (_, mut groups) = groups.into_iter().rev().fold(
            (HashSet::new(), Vec::new()),
            |(mut seen, mut fold), g| {
                let g = g.replace(|c| !seen.insert(c), "");
                fold.push(g);

                (seen, fold)
            },
        );
        groups.reverse();
        groups
    } else {
        groups.into_iter().map(|s| s.to_owned()).collect()
    };

    let groups = groups.into_iter().map(render_group).collect::<Vec<_>>();
    Ok(format!("{}Attack", groups.join("or ")))
}

fn render_hit_bonus_tag(args: Vec<&str>) -> Result<String> {
    check_arg_count(1.., args.len())?;

    if args.len() >= 2 {
        Ok(args[1].to_owned())
    } else {
        let n = args[0]
            .parse::<i64>()
            .map_err(|_| RenderErrorKind::ArgFormat("Could not parse argument as an integer."))?;
        Ok(format!("{:+}", n))
    }
}

fn render_recharge_tag(args: Vec<&str>) -> Result<String> {
    check_arg_count(0..=1, args.len())?;

    let as_num = args
        .first()
        .map(|s| s.parse::<u8>())
        .unwrap_or(Ok(6))
        .map_err(|_| RenderErrorKind::ArgFormat("Could not parse argument as an integer."))?;

    if as_num == 6 {
        Ok("(Recharge 6)".to_owned())
    } else {
        Ok(format!("(Recharge {}-6)", as_num))
    }
}

fn render_homebrew_tag(args: Vec<&str>) -> Result<String> {
    check_arg_count(1..=2, args.len())?;

    let (new_text, old_text) = (args[0], args.get(1).unwrap_or(&""));

    if !new_text.is_empty() && !old_text.is_empty() {
        Ok(format!(
            "{} [this is a homebrew addition, replacing the following: {}]",
            new_text, old_text
        ))
    } else if old_text.is_empty() {
        Ok(format!("{} [this is a homebrew addition]", new_text))
    } else if new_text.is_empty() {
        Ok(format!(
            "[the following text has been removed as part of a homebrew: {}]",
            old_text
        ))
    } else {
        Err(Error::from(RenderErrorKind::ArgFormat(
            "Homebrew tag had neither old nor new text.",
        )))
    }
}

fn render_area_tag(args: Vec<&str>) -> Result<String> {
    check_arg_count(1.., args.len())?;

    let (compact_text, flags) = (args[0], args.get(2).unwrap_or(&""));
    let flags = flags.chars().collect::<HashSet<_>>();

    if flags.contains(&'x') {
        Ok(compact_text.to_owned())
    } else if flags.contains(&'u') {
        Ok(format!("Area {}", compact_text))
    } else {
        Ok(format!("area {}", compact_text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(actual: Result<String>, expected: Result<&str>) {
        assert_eq!(actual, expected.map(String::from),);
    }

    #[test]
    fn attack_tag() {
        check(render_attack_tag(vec!["mw"]), Ok("Melee Weapon Attack"));

        check(
            render_attack_tag(vec!["mw,rs"]),
            Ok("Melee Weapon or Ranged Spell Attack"),
        );

        check(
            render_attack_tag(vec!["ms,rs"]),
            Ok("Melee or Ranged Spell Attack"),
        );
    }

    #[test]
    fn hit_tag() {
        check(
            render_hit_bonus_tag(vec!["7", "display text"]),
            Ok("display text"),
        );

        check(render_hit_bonus_tag(vec!["-7"]), Ok("-7"));

        check(render_hit_bonus_tag(vec!["7"]), Ok("+7"));
    }

    #[test]
    fn recharge_tag() {
        check(render_recharge_tag(vec!["4"]), Ok("(Recharge 4-6)"));

        check(render_recharge_tag(vec!["6"]), Ok("(Recharge 6)"));

        check(render_recharge_tag(vec![]), Ok("(Recharge 6)"));
    }

    #[test]
    fn homebrew_tag() {
        check(
            render_homebrew_tag(vec!["10d6", "8d6"]),
            Ok("10d6 [this is a homebrew addition, replacing the following: 8d6]"),
        );

        check(
            render_homebrew_tag(vec!["10d6", ""]),
            Ok("10d6 [this is a homebrew addition]"),
        );

        check(
            render_homebrew_tag(vec!["", "8d6"]),
            Ok("[the following text has been removed as part of a homebrew: 8d6]"),
        );

        check(
            render_homebrew_tag(vec!["10d6"]),
            Ok("10d6 [this is a homebrew addition]"),
        );
    }

    #[test]
    fn area_tag() {
        check(render_area_tag(vec!["5 feet"]), Ok("area 5 feet"));

        check(render_area_tag(vec!["5 feet", "", "xu"]), Ok("5 feet"));

        check(render_area_tag(vec!["5 feet", "", "u"]), Ok("Area 5 feet"));
    }
}
