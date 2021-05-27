use logos::{Logos, SpannedIter};
use std::iter::Peekable;

pub(crate) struct Lexer<'a> {
    inner: Peekable<SpannedIter<'a, Token>>,
    source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let inner = Token::lexer(source)
            .spanned()
            .peekable();

        Self {
            inner,
            source,
        }
    }

    fn next(&mut self) -> Option<Lexeme<'a>> {
        let (token, span) = match self.inner.next() {
            Some(v) => v,
            None => return None,
        };

        let ret = match token {
            Token::TagOpen => self.tag(),
            Token::TagClose | Token::Text => self.text(span.start),
            Token::Error => self.error(LexErrorKind::UnexpectedToken),
        };

        Some(ret)
    }

    fn tag(&mut self) -> Lexeme<'a> {
        let first_span = match self.inner.next() {
            Some((Token::Text, span)) => span,
            Some(_) => return self.error(LexErrorKind::NoTagName),
            None => return self.error(LexErrorKind::UnclosedTag),
        };

        let tag_name = match self.source[first_span.clone()].split_once(' ') {
            Some((s, _)) => s,
            None => &self.source[first_span.clone()],
        };

        let mut depth = 1usize;

        let end: usize = loop {
            let (token, span) = match self.inner.next() {
                Some(v) => v,
                None => return self.error(LexErrorKind::UnclosedTag),
            };

            match token {
                Token::TagOpen => depth += 1,
                Token::TagClose => {
                    depth -= 1;
                    if depth == 0 {
                        break span.end;
                    }
                }
                Token::Text => (),
                Token::Error => return self.error(LexErrorKind::UnexpectedToken),
            }
        };

        let args_start = first_span.start + tag_name.len() + 1;
        let args = if end <= args_start {
            None
        } else {
            Some(&self.source[args_start..end - 1])
        };

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
                Token::TagClose | Token::Text => {
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
        Lexeme::Error {
            kind,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Lexeme<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum Lexeme<'a> {
    Tag {
        name: &'a str,
        args: Option<&'a str>,
    },
    Text(&'a str),
    Error {
        kind: LexErrorKind,
    },
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum LexErrorKind {
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

    #[regex(r"(\{[^@]|[^{}])+")]
    Text,

    #[error]
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Range;

    fn tokenize(input: &str) -> Vec<(Token, Range<usize>)> {
        Token::lexer(input)
            .spanned()
            .collect()
    }

    fn spanned(tokens: Vec<(Token, &str)>) -> Vec<(Token, Range<usize>)> {
        let mut offset = 0usize;
        let mut ret = Vec::new();

        for (token, slice) in tokens {
            ret.push((token, offset..offset+slice.len()));
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
                (Token::Text, "spell fireball|phb"),
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
                (Token::Text, "spell fireball|phb"),
                (Token::TagClose, "}"),
                (Token::Text, " describes the fireball spell")
            ]),
        );
    }

    fn lex(input: &str) -> Vec<Lexeme> {
        Lexer::new(input)
            .into_iter()
            .collect()
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
            vec![Lexeme::Tag { name: "spell", args: Some("fireball|phb") }],
        );
    }

    #[test]
    fn lex_text_with_tag() {
        assert_eq!(
            lex("The tag {@spell fireball|phb} describes the fireball spell"),
            vec![
                Lexeme::Text("The tag "),
                Lexeme::Tag { name: "spell", args: Some("fireball|phb") },
                Lexeme::Text(" describes the fireball spell"),
            ],
        );
    }

    #[test]
    fn lex_tag_without_args() {
        assert_eq!(
            lex("{@h} 5 (1d8+1) necrotic damage"),
            vec![
                Lexeme::Tag { name: "h", args: None },
                Lexeme::Text(" 5 (1d8+1) necrotic damage"),
            ],
        );
    }
}
