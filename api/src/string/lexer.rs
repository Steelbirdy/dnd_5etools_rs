use super::Result;
use logos::{Logos, SpannedIter};
use std::iter::Peekable;
use std::ops::Range;
use thiserror::Error;

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

    fn next(&mut self) -> Option<Result<Lexeme<'a>>> {
        let (token, span) = match self.inner.next() {
            Some(v) => v,
            None => return None,
        };
        // TODO: Add an EscapedTagOpen token variant for "\{@"
        let ret = match token {
            Token::TagOpen => self.tag(span.start),
            Token::TagClose | Token::ArgSeparator | Token::Text => self.text(span.start),
            Token::Error => self.error(LexError::UnexpectedToken {
                token: self.slice(&span).to_string(),
                index: span.start,
            }),
        };

        Some(ret)
    }

    fn tag(&mut self, start: usize) -> Result<Lexeme<'a>> {
        // The span of the first token inside the tag. This will always span at least the tag's name.
        let first_span = match self.inner.next() {
            Some((Token::Text, span)) => span,
            Some(_) => {
                return self.error(LexError::NoTagName(start));
            }
            None => {
                return self.error(LexError::UnclosedTag(start));
            }
        };

        // Split the token at the first space to get the tag's name.
        let tag_name = match self.slice(&first_span).split_once(' ') {
            Some((s, _)) => s,
            None => self.slice(&first_span),
        };

        // We need to match the correct pair of brackets to account for nested tags.
        let mut depth = 1usize;
        let args_start = first_span.start + tag_name.len() + 1;
        let mut last_arg_start = args_start;

        let mut args = Vec::new();

        let end: usize = loop {
            let (token, span) = match self.inner.next() {
                Some(v) => v,
                None => return self.error(LexError::UnclosedTag(start)),
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
                            args.push(self.slice(&(last_arg_start..span.start)));
                        }
                        last_arg_start = span.end;
                    }
                }
                Token::Text => (),
                Token::Error => {
                    return self.error(LexError::UnexpectedToken {
                        token: self.slice(&span).to_string(),
                        index: span.start,
                    })
                }
            }
        };

        if last_arg_start >= end {
            if !args.is_empty() {
                args.push("");
            }
        } else {
            args.push(self.slice(&(last_arg_start..end - 1)));
        }

        Ok(Lexeme::tag(tag_name, args))
    }

    fn text(&mut self, start: usize) -> Result<Lexeme<'a>> {
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
                    let span = span.start..span.end;
                    let token = self.slice(&span).to_string();

                    return self.error(LexError::UnexpectedToken {
                        token,
                        index: span.start,
                    });
                }
            }
        };

        Ok(Lexeme::Text(self.slice(&(start..end))))
    }

    fn error<T>(&self, err: LexError) -> Result<T> {
        Err(err.into())
    }

    fn slice(&self, span: &Range<usize>) -> &'a str {
        &self.source[span.start..span.end]
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Lexeme<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lexeme<'a> {
    Tag(LexemeTag<'a>),
    Text(&'a str),
}

impl<'a> Lexeme<'a> {
    fn tag(name: &'a str, args: Vec<&'a str>) -> Self {
        Self::Tag(LexemeTag { name, args })
    }
}

/// Represents a tag that has been parsed into a lexeme.
/// Unlike [super::tags::Tag], this type allows 100% of the input space.
#[derive(Debug, Clone, PartialEq)]
pub struct LexemeTag<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
}

#[derive(Error, Debug, PartialEq)]
pub enum LexError {
    #[error("tag beginning at index {0} does not have a name")]
    NoTagName(usize),
    #[error("tag beginning at index {0} is never closed")]
    UnclosedTag(usize),
    #[error("unexpected token `{token}` at index `{index}`")]
    UnexpectedToken { token: String, index: usize },
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
        Lexer::new(input).map(|l| l.unwrap()).collect()
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
            vec![Lexeme::tag("spell", vec!["fireball", "phb"])],
        );
    }

    #[test]
    fn lex_text_with_tag() {
        assert_eq!(
            lex("The tag {@spell fireball|phb} describes the fireball spell"),
            vec![
                Lexeme::Text("The tag "),
                Lexeme::tag("spell", vec!["fireball", "phb"]),
                Lexeme::Text(" describes the fireball spell"),
            ],
        );
    }

    #[test]
    fn lex_tag_without_args() {
        assert_eq!(
            lex("{@h} 5 (1d8+1) necrotic damage"),
            vec![
                Lexeme::tag("h", vec![]),
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
                Lexeme::tag(
                    "class",
                    vec![
                        "",
                        "fighter",
                        "phb",
                        "",
                        "{@b eldritch knight}",
                        "",
                        "",
                        "phb",
                        ""
                    ]
                ),
                Lexeme::Text(" is a third-caster"),
            ],
        );
    }
}
