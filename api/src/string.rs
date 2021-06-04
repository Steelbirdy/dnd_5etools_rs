mod error;
mod lexer;
mod render;
mod tags;

pub use error::*;
pub use lexer::Lexeme;
pub use render::{DefaultStringRenderer, RenderString};
pub use tags::{Tag, TagError, TagName};

#[allow(dead_code)]
pub fn render<R: RenderString>(renderer: R, input: &str) -> Result<String> {
    renderer.render(input)
}

pub fn tokenize(input: &str) -> Result<impl Iterator<Item = Lexeme>> {
    Ok(lexer::Lexer::new(input)
        .collect::<Result<Vec<_>>>()?
        .into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check<R: RenderString>(renderer: R, input: &str, expected: Result<&str>) {
        assert_eq!(renderer.render(input), expected.map(String::from),)
    }

    fn check_def(input: &str, expected: Result<&str>) {
        check(DefaultStringRenderer, input, expected)
    }

    #[test]
    fn render_string_no_tags() {
        check_def("Hello, world!", Ok("Hello, world!"));
    }

    #[test]
    fn render_tag() {
        check_def(
            "{@b This text is unchanged.}",
            Ok("This text is unchanged."),
        );

        check_def(
            "{@chance 100} {@chance 25|this is displayed.}",
            Ok("100% this is displayed."),
        );
    }

    #[test]
    fn render_long_entry() {
        check_def(
            "A {@condition restrained} creature can make a Dexterity saving throw at the end of each of \
            its turns, ending the effect on itself on a success. Alternatively, the creature or someone else who can \
            reach it can use an action to make an Intelligence ({@skill Arcana}) check against your spell save DC. On \
            a success, the {@condition restrained|phb|restraints} disappear.",
            Ok("A restrained creature can make a Dexterity saving throw at the end of each of \
            its turns, ending the effect on itself on a success. Alternatively, the creature or someone else who can \
            reach it can use an action to make an Intelligence (Arcana) check against your spell save DC. On \
            a success, the restraints disappear."),
        );
    }
}
