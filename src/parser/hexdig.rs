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

/// HEXDIG         =  DIGIT / "A" / "B" / "C" / "D" / "E" / "F"
///
/// DIGIT          =  %x30-39
///                        ; 0-9
///
/// <https://datatracker.ietf.org/doc/html/rfc2234#section-6.1>
pub fn hexdig(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) =
        nom::character::complete::satisfy(|c| matches!(c, '0'..='9') || matches!(c, 'A'..='F'))
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
    fn test_hexdig() {
        ok(hexdig, "0rest", ("rest", "0"));
        ok(hexdig, "1rest", ("rest", "1"));
        ok(hexdig, "2rest", ("rest", "2"));
        ok(hexdig, "3rest", ("rest", "3"));
        ok(hexdig, "4rest", ("rest", "4"));
        ok(hexdig, "5rest", ("rest", "5"));
        ok(hexdig, "6rest", ("rest", "6"));
        ok(hexdig, "7rest", ("rest", "7"));
        ok(hexdig, "8rest", ("rest", "8"));
        ok(hexdig, "9rest", ("rest", "9"));
        ok(hexdig, "Arest", ("rest", "A"));
        ok(hexdig, "Brest", ("rest", "B"));
        ok(hexdig, "Crest", ("rest", "C"));
        ok(hexdig, "Drest", ("rest", "D"));
        ok(hexdig, "Erest", ("rest", "E"));
        ok(hexdig, "Frest", ("rest", "F"));

        err(hexdig, "Grest");
        err(hexdig, "grest");
        err(hexdig, "arest");
        err(hexdig, "!rest");
        err(hexdig, "");
    }
}
