use api::string;
use api::string::{Lexeme, Result, RenderString};
use api::string::render_utils;
use std::convert::TryInto;

struct StringRenderer;

impl string::RenderString for StringRenderer {
    fn render(&self, input: &str) -> Result<String> {
        string::tokenize(input)?
            .map(|lexeme| match lexeme {
                Lexeme::Tag(lexeme_tag) => self.render_tag(lexeme_tag.try_into()?),
                Lexeme::Text(text) => Ok(text.to_owned()),
            })
            .collect()
    }

    fn render_bold(&self, args: Vec<&str>) -> Result<String> {
        render_utils::check_arg_count(1..=1, args.len())?;
        Ok(format!("**{}**", self.render(args[0])?))
    }

    fn render_italic(&self, args: Vec<&str>) -> Result<String> {
        render_utils::check_arg_count(1..=1, args.len())?;
        Ok(format!("*{}*", self.render(args[0])?))
    }

    fn render_strikethrough(&self, args: Vec<&str>) -> Result<String> {
        render_utils::check_arg_count(1..=1, args.len())?;
        Ok(format!("~~{}~~", self.render(args[0])?))
    }

    fn render_underline(&self, args: Vec<&str>) -> Result<String> {
        render_utils::check_arg_count(1..=1, args.len())?;
        Ok(format!("__{}__", self.render(args[0])?))
    }
}

#[test]
fn render_simple_formatting_tags() {
    let input = "It is {@b very} easy to render {@s trivial} {@u {@i simple}} markdown with these tags.";
    let expected = "It is **very** easy to render ~~trivial~~ __*simple*__ markdown with these tags.";

#[test]
fn error_on_invalid_args() {
    let input = "Trying to render a tag with {@b too|many} arguments returns an error.";
    let expected = RenderError::arg_count(1..=1, 2);

    assert_eq!(StringRenderer.render(input), Err(expected.into()));
}
