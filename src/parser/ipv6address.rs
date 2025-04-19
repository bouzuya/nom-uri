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

/// IPv6address =                            6( h16 ":" ) ls32
///             /                       "::" 5( h16 ":" ) ls32
///             / [               h16 ] "::" 4( h16 ":" ) ls32
///             / [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
///             / [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
///             / [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
///             / [ *4( h16 ":" ) h16 ] "::"              ls32
///             / [ *5( h16 ":" ) h16 ] "::"              h16
///             / [ *6( h16 ":" ) h16 ] "::"
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.2>
pub fn ipv6address(i: Span) -> IResult<Span, Token> {
    fn f<'a, F>(i: Span<'a>, mut parser: F, tag: &'static str) -> IResult<Span<'a>, Span<'a>>
    where
        F: nom::Parser<Span<'a>, Error = nom::error::Error<Span<'a>>>,
    {
        use nom::error::ParseError;

        let start = i.clone();
        let (i, i2) = nom::bytes::take_until(tag).parse(i)?;
        let (i3, _) = parser.parse(i2)?;
        if i3.input_len() != 0 {
            return Err(nom::Err::Error(
                <F as Parser<Span<'a>>>::Error::from_error_kind(
                    start,
                    nom::error::ErrorKind::ManyMN,
                ),
            ));
        }
        Ok((i.clone(), start.take(start.offset(&i))))
    }

    let start = i;
    let (i, _) = nom::branch::alt((
        // =                            6( h16 ":" ) ls32
        (
            nom::multi::count((h16, nom::character::complete::char(':')), 6),
            ls32,
        )
            .map(|_| ()),
        // /                       "::" 5( h16 ":" ) ls32
        (
            nom::bytes::complete::tag("::"),
            nom::multi::count((h16, nom::character::complete::char(':')), 5),
            ls32,
        )
            .map(|_| ()),
        // / [               h16 ] "::" 4( h16 ":" ) ls32
        (
            |i| f(i, nom::combinator::opt(h16), "::"),
            // nom::combinator::opt(h16),
            nom::bytes::complete::tag("::"),
            nom::multi::count((h16, nom::character::complete::char(':')), 4),
            ls32,
        )
            .map(|_| ()),
        // / [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
        (
            nom::bytes::complete::tag("::"),
            nom::multi::count((h16, nom::character::complete::char(':')), 3),
            ls32,
        )
            .map(|_| ()),
        (
            |i| {
                f(
                    i,
                    (
                        nom::multi::many_m_n(0, 1, (h16, nom::character::complete::char(':'))),
                        h16,
                    ),
                    "::",
                )
            },
            nom::bytes::complete::tag("::"),
            nom::multi::count((h16, nom::character::complete::char(':')), 3),
            ls32,
        )
            .map(|_| ()),
        // / [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
        (
            nom::bytes::complete::tag("::"),
            nom::multi::count((h16, nom::character::complete::char(':')), 2),
            ls32,
        )
            .map(|_| ()),
        (
            |i| {
                f(
                    i,
                    (
                        nom::multi::many_m_n(0, 2, (h16, nom::character::complete::char(':'))),
                        h16,
                    ),
                    "::",
                )
            },
            nom::bytes::complete::tag("::"),
            nom::multi::count((h16, nom::character::complete::char(':')), 2),
            ls32,
        )
            .map(|_| ()),
        // / [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
        (
            nom::bytes::complete::tag("::"),
            (h16, nom::character::complete::char(':')),
            ls32,
        )
            .map(|_| ()),
        (
            |i| {
                f(
                    i,
                    (
                        nom::multi::many_m_n(0, 3, (h16, nom::character::complete::char(':'))),
                        h16,
                    ),
                    "::",
                )
            },
            nom::bytes::complete::tag("::"),
            (h16, nom::character::complete::char(':')),
            ls32,
        )
            .map(|_| ()),
        // / [ *4( h16 ":" ) h16 ] "::"              ls32
        (nom::bytes::complete::tag("::"), ls32).map(|_| ()),
        (
            |i| {
                f(
                    i,
                    (
                        nom::multi::many_m_n(0, 4, (h16, nom::character::complete::char(':'))),
                        h16,
                    ),
                    "::",
                )
            },
            nom::bytes::complete::tag("::"),
            ls32,
        )
            .map(|_| ()),
        // / [ *5( h16 ":" ) h16 ] "::"              h16
        (nom::bytes::complete::tag("::"), h16).map(|_| ()),
        (
            |i| {
                f(
                    i,
                    (
                        nom::multi::many_m_n(0, 5, (h16, nom::character::complete::char(':'))),
                        h16,
                    ),
                    "::",
                )
            },
            nom::bytes::complete::tag("::"),
            h16,
        )
            .map(|_| ()),
        // / [ *6( h16 ":" ) h16 ] "::"
        nom::bytes::complete::tag("::").map(|_| ()),
        (
            |i| {
                f(
                    i,
                    (
                        nom::multi::many_m_n(0, 6, (h16, nom::character::complete::char(':'))),
                        h16,
                    ),
                    "::",
                )
            },
            nom::bytes::complete::tag("::"),
        )
            .map(|_| ()),
    ))
    .parse(i)?;
    Ok((
        i,
        Token {
            span: start.take(start.offset(&i)),
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::parser::tests::{err, ok};

    use super::*;

    #[test]
    fn test_ipv6address() {
        // 6( h16 ":" ) ls32
        ok(
            ipv6address,
            "1111:2222:3333:4444:5555:6666:7777:8888",
            ("", "1111:2222:3333:4444:5555:6666:7777:8888"),
        );
        ok(
            ipv6address,
            "1111:2222:3333:4444:5555:6666:127.0.0.1",
            ("", "1111:2222:3333:4444:5555:6666:127.0.0.1"),
        );

        // "::" 5( h16 ":" ) ls32
        ok(
            ipv6address,
            "::1111:2222:3333:4444:5555:6666:7777",
            ("", "::1111:2222:3333:4444:5555:6666:7777"),
        );
        ok(
            ipv6address,
            "::1111:2222:3333:4444:5555:192.168.0.1",
            ("", "::1111:2222:3333:4444:5555:192.168.0.1"),
        );

        // [ h16 ] "::" 4( h16 ":" ) ls32
        ok(
            ipv6address,
            "1111::2222:3333:4444:5555:6666:7777",
            ("", "1111::2222:3333:4444:5555:6666:7777"),
        );
        ok(
            ipv6address,
            "1111::2222:3333:4444:5555:127.0.0.1",
            ("", "1111::2222:3333:4444:5555:127.0.0.1"),
        );
        ok(
            ipv6address,
            "::2222:3333:4444:5555:6666:7777",
            ("", "::2222:3333:4444:5555:6666:7777"),
        );

        // [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
        ok(
            ipv6address,
            "1111:2222::3333:4444:5555:6666:7777",
            ("", "1111:2222::3333:4444:5555:6666:7777"),
        );
        ok(
            ipv6address,
            "1111:2222::3333:4444:5555:127.0.0.1",
            ("", "1111:2222::3333:4444:5555:127.0.0.1"),
        );
        ok(
            ipv6address,
            "::3333:4444:5555:6666:7777",
            ("", "::3333:4444:5555:6666:7777"),
        );

        // [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
        ok(
            ipv6address,
            "1111:2222:3333::4444:5555:6666:7777",
            ("", "1111:2222:3333::4444:5555:6666:7777"),
        );
        ok(
            ipv6address,
            "1111:2222:3333::4444:5555:127.0.0.1",
            ("", "1111:2222:3333::4444:5555:127.0.0.1"),
        );
        ok(
            ipv6address,
            "::4444:5555:6666:7777",
            ("", "::4444:5555:6666:7777"),
        );

        // [ *3( h16 ":" ) h16 ] "::" h16 ":" ls32
        ok(
            ipv6address,
            "1111:2222:3333:4444::5555:6666:7777",
            ("", "1111:2222:3333:4444::5555:6666:7777"),
        );
        ok(
            ipv6address,
            "1111:2222:3333:4444::5555:127.0.0.1",
            ("", "1111:2222:3333:4444::5555:127.0.0.1"),
        );
        ok(ipv6address, "::5555:6666:7777", ("", "::5555:6666:7777"));

        // [ *4( h16 ":" ) h16 ] "::" ls32
        ok(
            ipv6address,
            "1111:2222:3333:4444:5555::6666:7777",
            ("", "1111:2222:3333:4444:5555::6666:7777"),
        );
        ok(
            ipv6address,
            "1111:2222:3333:4444:5555::127.0.0.1",
            ("", "1111:2222:3333:4444:5555::127.0.0.1"),
        );
        ok(ipv6address, "::6666:7777", ("", "::6666:7777"));
        ok(ipv6address, "::127.0.0.1", ("", "::127.0.0.1"));

        // [ *5( h16 ":" ) h16 ] "::" h16
        ok(
            ipv6address,
            "1111:2222:3333:4444:5555:6666::7777",
            ("", "1111:2222:3333:4444:5555:6666::7777"),
        );
        ok(ipv6address, "::7777", ("", "::7777"));

        // [ *6( h16 ":" ) h16 ] "::"
        ok(
            ipv6address,
            "1111:2222:3333:4444:5555:6666:7777::",
            ("", "1111:2222:3333:4444:5555:6666:7777::"),
        );
        ok(ipv6address, "::", ("", "::"));

        // Test case sensitivity
        ok(
            ipv6address,
            "abcd:EF01:2345:6789:abcd:EF01:2345:6789",
            ("", "abcd:EF01:2345:6789:abcd:EF01:2345:6789"),
        );

        // Test with trailing characters
        // ok(ipv6address, "::7777/64", ("/64", "::7777"));
        ok(
            ipv6address,
            // "2001:db8::1",
            "2001:db8::1:0:0:1/64",
            // "2001:db8::1:/",
            // ("", ""),
            ("/64", "2001:db8::1:0:0:1"),
        );

        ok(ipv6address, "2001:db8:::1", (":1", "2001:db8::"));
        ok(ipv6address, "2001:db8::1::", ("::", "2001:db8::1"));
        ok(ipv6address, ":::1", (":1", "::"));
        ok(ipv6address, "1:::", (":", "1::"));
        ok(ipv6address, "1:2:3:4:5:6:7:8:9", (":9", "1:2:3:4:5:6:7:8"));
        err(ipv6address, "1:2:3");
    }

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

        fn h3(i: Span) -> IResult<Span, Token> {
            let start = i;
            let (i, _) = Lookahead::new(
                (
                    nom::multi::many_m_n(0, 2, (h16, nom::character::complete::char(':'))),
                    h16,
                ),
                "::",
            )
            .parse(i)?;
            Ok((
                i,
                Token {
                    span: start.take(start.offset(&i)),
                },
            ))
        }

        err(h3, "1:2:3:4:5::");
        err(h3, "1:2:3:4::");
        ok(h3, "1:2:3::", ("", "1:2:3::"));
        ok(h3, "1:2::", ("", "1:2::"));
        ok(h3, "1::", ("", "1::"));

        fn h4_<'a, F>(i: Span<'a>, parser: F, t: &'static str) -> IResult<Span<'a>, Token<'a>>
        where
            F: nom::Parser<Span<'a>, Error = nom::error::Error<Span<'a>>>,
        {
            let start = i;
            let (i, _) = Lookahead::new(parser, t).parse(i)?;
            Ok((
                i,
                Token {
                    span: start.take(start.offset(&i)),
                },
            ))
        }

        fn h4(i: Span) -> IResult<Span, Token> {
            h4_(
                i,
                (
                    nom::multi::many_m_n(0, 2, (h16, nom::character::complete::char(':'))),
                    h16,
                ),
                "::",
            )
        }

        err(h4, "1:2:3:4:5::");
        err(h4, "1:2:3:4::");
        ok(h4, "1:2:3::", ("", "1:2:3::"));
        ok(h4, "1:2::", ("", "1:2::"));
        ok(h4, "1::", ("", "1::"));
    }
}
