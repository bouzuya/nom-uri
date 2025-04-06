use nom::{IResult, Input as _, Offset as _, Parser};

use super::{HasSpan, Span};
use crate::parser::dec_octet;

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub span: Span<'a>,
}

impl<'a> HasSpan<'a> for Token<'a> {
    fn span(&self) -> Span<'a> {
        self.span
    }
}

/// IPv4address = dec-octet "." dec-octet "." dec-octet "." dec-octet
///
/// <https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.2>
pub fn ipv4address(i: Span) -> IResult<Span, Token> {
    let start = i;
    let (i, _) = (
        dec_octet,
        nom::character::complete::char('.'),
        dec_octet,
        nom::character::complete::char('.'),
        dec_octet,
        nom::character::complete::char('.'),
        dec_octet,
    )
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
    fn test_ipv4address() {
        ok(ipv4address, "192.168.0.1", ("", "192.168.0.1"));
        ok(ipv4address, "127.0.0.1", ("", "127.0.0.1"));
        ok(ipv4address, "0.0.0.0", ("", "0.0.0.0"));
        ok(ipv4address, "255.255.255.255", ("", "255.255.255.255"));
        ok(ipv4address, "1.2.3.4", ("", "1.2.3.4"));

        ok(ipv4address, "192.168.0.1:8080", (":8080", "192.168.0.1"));
        ok(ipv4address, "192.168.0.1/24", ("/24", "192.168.0.1"));
        ok(ipv4address, "192.168.0.1?query", ("?query", "192.168.0.1"));
        ok(
            ipv4address,
            "192.168.0.1#fragment",
            ("#fragment", "192.168.0.1"),
        );

        err(ipv4address, "");
        err(ipv4address, "192.168.0");
        err(ipv4address, "192.168.0.");
        err(ipv4address, "192.168..1");
        ok(ipv4address, "192.168.0.1.5", (".5", "192.168.0.1"));
        err(ipv4address, "256.0.0.0");
        ok(ipv4address, "192.168.0.256", ("6", "192.168.0.25"));
        ok(ipv4address, "192.168.0.01", ("1", "192.168.0.0"));
        err(ipv4address, "a.b.c.d");
        err(ipv4address, "192,168,0,1");
        err(ipv4address, "-1.2.3.4");
        err(ipv4address, "3232235521");
        err(ipv4address, "0x7f.0x0.0x0.0x1");
    }
}
