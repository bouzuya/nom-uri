use nom::{IResult, Input as _, Offset as _, Parser};

use super::{HasSpan, Span};
use crate::parser::{h16, ls32};

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub span: Span<'a>,
}

impl<'a> HasSpan<'a> for Token<'a> {
    fn span(&self) -> Span<'a> {
        self.span
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tests::{err, ok};

    use super::*;

    #[test]
    fn test_lookahead() {
        fn f(i: Span) -> IResult<Span, Token> {
            let start = i;
            let (i, _) =
                nom::multi::many_m_n(0, 2, (h16, nom::character::complete::char(':'))).parse(i)?;
            Ok((
                i,
                Token {
                    span: start.take(start.offset(&i)),
                },
            ))
        }

        ok(f, "1:2:3", ("3", "1:2:"));
        ok(f, "1:2", ("2", "1:"));
        ok(f, "1", ("1", ""));

        fn g(i: Span) -> IResult<Span, Token> {
            let start = i;
            let (i, _) = (
                nom::multi::many_m_n(0, 2, (h16, nom::character::complete::char(':'))),
                h16,
            )
                .parse(i)?;
            Ok((
                i,
                Token {
                    span: start.take(start.offset(&i)),
                },
            ))
        }

        ok(g, "1:2:3", ("", "1:2:3"));
        ok(g, "1:2", ("", "1:2"));
        ok(g, "1", ("", "1"));

        fn h(i: Span) -> IResult<Span, Token> {
            let start = i;
            let (i, _) = (
                nom::multi::many_m_n(0, 2, (h16, nom::character::complete::char(':'))),
                h16,
                nom::bytes::complete::tag("::"),
            )
                .parse(i)?;
            Ok((
                i,
                Token {
                    span: start.take(start.offset(&i)),
                },
            ))
        }

        ok(h, "1:2:3::", ("", "1:2:3::"));
        err(h, "1:2::"); // ...
        err(h, "1::"); // ...

        fn h2(i: Span) -> IResult<Span, Token> {
            let start = i;
            let (i, i2) = nom::bytes::take_until("::").parse(i)?;
            let (i3, _) = (
                nom::multi::many_m_n(0, 2, (h16, nom::character::complete::char(':'))),
                h16,
            )
                .parse(i2)?;
            if !i3.is_empty() {
                return Err(nom::Err::Error(nom::error::Error::new(
                    start,
                    nom::error::ErrorKind::ManyMN,
                )));
            }
            let (i, _) = nom::bytes::complete::tag("::").parse(i)?;
            Ok((
                i,
                Token {
                    span: start.take(start.offset(&i)),
                },
            ))
        }

        err(h2, "1:2:3:4:5::");
        err(h2, "1:2:3:4::");
        ok(h2, "1:2:3::", ("", "1:2:3::"));
        ok(h2, "1:2::", ("", "1:2::"));
        ok(h2, "1::", ("", "1::"));

        struct Lookahead<F> {
            parser: F,
            tag: &'static str,
        }

        impl<F> Lookahead<F> {
            fn new(parser: F, tag: &'static str) -> Self {
                Lookahead { parser, tag }
            }
        }

        impl<I, F> nom::Parser<I> for Lookahead<F>
        where
            I: Clone
                + nom::Input
                + nom::Compare<&'static str>
                + nom::FindSubstring<&'static str>
                + nom::Offset,
            F: nom::Parser<I>,
        {
            type Output = I;

            type Error = <F as Parser<I>>::Error;

            fn parse(&mut self, input: I) -> IResult<I, Self::Output, Self::Error> {
                use nom::error::ParseError;

                let start = input.clone();
                let (i, i2) = nom::bytes::take_until(self.tag).parse(input)?;
                let (i3, _) = self.parser.parse(i2)?;
                if i3.input_len() != 0 {
                    return Err(nom::Err::Error(<F as Parser<I>>::Error::from_error_kind(
                        start,
                        nom::error::ErrorKind::ManyMN,
                    )));
                }
                let (i, _) = nom::bytes::complete::tag(self.tag).parse(i)?;
                Ok((i.clone(), start.take(start.offset(&i))))
            }

            fn process<OM: nom::OutputMode>(
                &mut self,
                _input: I,
            ) -> nom::PResult<OM, I, Self::Output, Self::Error> {
                unimplemented!()
            }
        }

        // TODO: test
    }
}
