use nom::{IResult, Input as _, Offset as _, Parser};

use super::{HasSpan, Span};

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub span: Span<'a>,
}

impl<'a> HasSpan<'a> for Token<'a> {
    fn span(&self) -> Span<'a> {
        self.span
    }
}

/// dec-octet     = DIGIT                 ; 0-9
///               / %x31-39 DIGIT         ; 10-99
///               / "1" 2DIGIT            ; 100-199
///               / "2" %x30-34 DIGIT     ; 200-249
///               / "25" %x30-35          ; 250-255
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.2>
pub fn dec_octet(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = nom::branch::alt((
        (
            nom::character::complete::char('2'),
            nom::character::complete::char('5'),
            nom::character::complete::satisfy(|c| matches!(c, '0'..='5')),
        )
            .map(|_| ()),
        (
            nom::character::complete::char('2'),
            nom::character::complete::satisfy(|c| matches!(c, '0'..='4')),
            nom::character::complete::satisfy(|c| c.is_ascii_digit()),
        )
            .map(|_| ()),
        (
            nom::character::complete::char('1'),
            nom::character::complete::satisfy(|c| c.is_ascii_digit()),
            nom::character::complete::satisfy(|c| c.is_ascii_digit()),
        )
            .map(|_| ()),
        (
            nom::character::complete::satisfy(|c| matches!(c, '1'..='9')),
            nom::character::complete::satisfy(|c| c.is_ascii_digit()),
        )
            .map(|_| ()),
        nom::character::complete::satisfy(|c| c.is_ascii_digit()).map(|_| ()),
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
    use crate::parser::tests::ok;

    use super::*;

    #[test]
    fn test_dec_octet() {
        ok(dec_octet, "0", ("", "0"));
        ok(dec_octet, "1", ("", "1"));
        ok(dec_octet, "9", ("", "9"));
        ok(dec_octet, "10", ("", "10"));
        ok(dec_octet, "99", ("", "99"));
        ok(dec_octet, "100", ("", "100"));
        ok(dec_octet, "199", ("", "199"));
        ok(dec_octet, "200", ("", "200"));
        ok(dec_octet, "249", ("", "249"));
        ok(dec_octet, "250", ("", "250"));
        ok(dec_octet, "255", ("", "255"));

        ok(dec_octet, "01", ("1", "0"));
        ok(dec_octet, "256", ("6", "25"));
        ok(dec_octet, "260", ("0", "26"));
        ok(dec_octet, "300", ("0", "30"));
    }
}
