mod authority;
mod dec_octet;
mod fragment;
mod h16;
mod hexdig;
mod hier_part;
mod host;
mod ip_literal;
mod ipv4address;
mod ipv6address;
mod ipvfuture;
mod ls32;
mod path;
mod path_abempty;
mod path_absolute;
mod path_empty;
mod path_noscheme;
mod path_rootless;
mod pchar;
mod pct_encoded;
mod port;
mod query;
mod reg_name;
mod scheme;
mod segment;
mod segment_nz;
mod segment_nz_nc;
mod sub_delims;
mod unreserved;
mod uri;
mod userinfo;

use nom_locate::LocatedSpan;

pub(crate) type Span<'a> = LocatedSpan<&'a str>;
pub(crate) trait HasSpan<'a> {
    fn span(&self) -> Span<'a>;
}

pub use self::authority::authority;
pub use self::dec_octet::dec_octet;
pub use self::fragment::fragment;
pub use self::h16::h16;
pub use self::hexdig::hexdig;
pub use self::hier_part::hier_part;
pub use self::host::host;
pub use self::ip_literal::ip_literal;
pub use self::ipv4address::ipv4address;
pub use self::ipv6address::ipv6address;
pub use self::ipvfuture::ipvfuture;
pub use self::ls32::ls32;
pub use self::path_abempty::path_abempty;
pub use self::path_absolute::path_absolute;
pub use self::path_empty::path_empty;
pub use self::path_noscheme::path_noscheme;
pub use self::path_rootless::path_rootless;
pub use self::pchar::pchar;
pub use self::pct_encoded::pct_encoded;
pub use self::port::port;
pub use self::query::query;
pub use self::reg_name::reg_name;
pub use self::scheme::scheme;
pub use self::segment::segment;
pub use self::segment_nz::segment_nz;
pub use self::segment_nz_nc::segment_nz_nc;
pub use self::sub_delims::sub_delims;
pub use self::unreserved::unreserved;
pub use self::uri::uri;

#[cfg(test)]
mod tests {
    use nom::IResult;

    use super::*;

    pub(crate) fn err<'a, T>(f: fn(Span<'a>) -> IResult<Span<'a>, T>, s: &'a str) {
        use nom_locate::LocatedSpan;

        let s = LocatedSpan::new(s);
        assert!(f(s).is_err());
    }

    pub(crate) fn ok<'a, T: HasSpan<'a>>(
        f: fn(Span<'a>) -> IResult<Span<'a>, T>,
        s: &'a str,
        (i, o): (&'a str, &'a str),
    ) {
        use nom::Input;
        use nom_locate::LocatedSpan;

        let s = LocatedSpan::new(s);
        let expected = (
            s.take_from(s.rfind(i).expect("i")),
            s.take_from(s.find(o).expect("o")).take(o.len()),
        );
        assert_eq!(f(s).map(|(i, o)| (i, o.span())).expect("f"), expected);
    }
}
