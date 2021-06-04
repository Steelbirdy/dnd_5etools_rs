use logos::{Logos, SpannedIter};
use std::iter::Peekable;

/// Tokenize a string. This non-recursively separates tags within the string from raw text.
#[allow(dead_code)]
#[inline]
pub fn lex_string(source: &str) -> impl Iterator<Item = Lexeme<'_>> {
    Lexer::new(source)
}

/// Wrapper around logos::Lexer<'a, Token>
pub(crate) struct Lexer<'a> {
    inner: Peekable<SpannedIter<'a, Token>>,
    source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let inner = Token::lexer(source).spanned().peekable();

        Self { inner, source }
    }

    fn next(&mut self) -> Option<Lexeme<'a>> {
        let (token, span) = match self.inner.next() {
            Some(v) => v,
            None => return None,
        };
        // TODO: Add an EscapedTagOpen token variant for "\{@"
        let ret = match token {
            Token::TagOpen => self.tag(),
            Token::TagClose | Token::ArgSeparator | Token::Text => self.text(span.start),
            Token::Error => self.error(LexErrorKind::UnexpectedToken),
        };

        Some(ret)
    }

    fn tag(&mut self) -> Lexeme<'a> {
        // The span of the first token inside the tag. This will always span at least the tag's name.
        let first_span = match self.inner.next() {
            Some((Token::Text, span)) => span,
            Some(_) => return self.error(LexErrorKind::NoTagName),
            None => return self.error(LexErrorKind::UnclosedTag),
        };

        // Split the token at the first space to get the tag's name.
        let tag_name = match self.source[first_span.clone()].split_once(' ') {
            Some((s, _)) => s,
            None => &self.source[first_span.clone()],
        };

        // We need to match the correct pair of brackets to account for nested tags.
        let mut depth = 1usize;
        let args_start = first_span.start + tag_name.len() + 1;
        let mut last_arg_start = args_start;

        let mut args = Vec::new();

        let end: usize = loop {
            let (token, span) = match self.inner.next() {
                Some(v) => v,
                None => return self.error(LexErrorKind::UnclosedTag),
            };

            match token {
                // Entering a nested tag
                Token::TagOpen => depth += 1,
                // Leaving a tag
                Token::TagClose => {
                    depth -= 1;
                    if depth == 0 {
                        break span.end;
                    }
                }
                Token::ArgSeparator => {
                    if depth == 1 {
                        if last_arg_start >= span.start {
                            args.push("");
                        } else {
                            args.push(&self.source[last_arg_start..span.start]);
                        }
                        last_arg_start = span.end;
                    }
                }
                Token::Text => (),
                Token::Error => return self.error(LexErrorKind::UnexpectedToken),
            }
        };

        if last_arg_start >= end {
            if !args.is_empty() {
                args.push("");
            }
        } else {
            args.push(&self.source[last_arg_start..end - 1]);
        }

        Lexeme::Tag {
            name: tag_name,
            args,
        }
    }

    fn text(&mut self, start: usize) -> Lexeme<'a> {
        let end = loop {
            let (token, span) = match self.inner.peek() {
                Some(v) => v,
                None => break self.source.len(),
            };

            match token {
                Token::TagOpen => {
                    break span.start;
                }
                Token::TagClose | Token::ArgSeparator | Token::Text => {
                    self.inner.next();
                }
                Token::Error => {
                    return self.error(LexErrorKind::UnexpectedToken);
                }
            }
        };

        Lexeme::Text(&self.source[start..end])
    }

    fn error(&mut self, kind: LexErrorKind) -> Lexeme<'a> {
        Lexeme::Error(kind)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Lexeme<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lexeme<'a> {
    Tag { name: &'a str, args: Vec<&'a str> },
    Text(&'a str),
    Error(LexErrorKind),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LexErrorKind {
    NoTagName,
    UnclosedTag,
    UnexpectedToken,
}

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
pub(crate) enum Token {
    #[token("{@")]
    TagOpen,

    #[token("}")]
    TagClose,

    #[token("|")]
    ArgSeparator,

    #[regex(r"(\{[^@]|[^{}|])+")]
    Text,

    #[error]
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Range;

    fn tokenize(input: &str) -> Vec<(Token, Range<usize>)> {
        Token::lexer(input).spanned().collect()
    }

    fn spanned(tokens: Vec<(Token, &str)>) -> Vec<(Token, Range<usize>)> {
        let mut offset = 0usize;
        let mut ret = Vec::new();

        for (token, slice) in tokens {
            ret.push((token, offset..offset + slice.len()));
            offset += slice.len();
        }
        ret
    }

    #[test]
    fn tokenize_no_tags() {
        assert_eq!(
            tokenize("This is just a normal string"),
            spanned(vec![(Token::Text, "This is just a normal string")]),
        );
    }

    #[test]
    fn tokenize_only_tag() {
        assert_eq!(
            tokenize("{@spell fireball|phb}"),
            spanned(vec![
                (Token::TagOpen, "{@"),
                (Token::Text, "spell fireball"),
                (Token::ArgSeparator, "|"),
                (Token::Text, "phb"),
                (Token::TagClose, "}"),
            ]),
        );
    }

    #[test]
    fn tokenize_text_with_tag() {
        assert_eq!(
            tokenize("The tag {@spell fireball|phb} describes the fireball spell"),
            spanned(vec![
                (Token::Text, "The tag "),
                (Token::TagOpen, "{@"),
                (Token::Text, "spell fireball"),
                (Token::ArgSeparator, "|"),
                (Token::Text, "phb"),
                (Token::TagClose, "}"),
                (Token::Text, " describes the fireball spell")
            ]),
        );
    }

    fn lex(input: &str) -> Vec<Lexeme> {
        Lexer::new(input).into_iter().collect()
    }

    #[test]
    fn lex_no_tags() {
        assert_eq!(
            lex("This is just a normal string"),
            vec![Lexeme::Text("This is just a normal string")],
        );
    }

    #[test]
    fn lex_only_tag() {
        assert_eq!(
            lex("{@spell fireball|phb}"),
            vec![Lexeme::Tag {
                name: "spell",
                args: vec!["fireball", "phb"]
            }],
        );
    }

    #[test]
    fn lex_text_with_tag() {
        assert_eq!(
            lex("The tag {@spell fireball|phb} describes the fireball spell"),
            vec![
                Lexeme::Text("The tag "),
                Lexeme::Tag {
                    name: "spell",
                    args: vec!["fireball", "phb"]
                },
                Lexeme::Text(" describes the fireball spell"),
            ],
        );
    }

    #[test]
    fn lex_tag_without_args() {
        assert_eq!(
            lex("{@h} 5 (1d8+1) necrotic damage"),
            vec![
                Lexeme::Tag {
                    name: "h",
                    args: vec![]
                },
                Lexeme::Text(" 5 (1d8+1) necrotic damage"),
            ],
        );
    }

    #[test]
    fn lex_tag_with_empty_args() {
        assert_eq!(
            lex("The {@class |fighter|phb||{@b eldritch knight}|||phb|} is a third-caster"),
            vec![
                Lexeme::Text("The "),
                Lexeme::Tag {
                    name: "class",
                    args: vec![
                        "",
                        "fighter",
                        "phb",
                        "",
                        "{@b eldritch knight}",
                        "",
                        "",
                        "phb",
                        ""
                    ],
                },
                Lexeme::Text(" is a third-caster"),
            ],
        );
    }
}
