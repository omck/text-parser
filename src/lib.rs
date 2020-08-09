use core::iter::Peekable;
use core::str::{CharIndices, FromStr};

#[inline]
#[allow(dead_code)]
pub fn consume<'p, 's: 'p>
(source: &'s str, parser: &'p mut Peekable<CharIndices>, until: fn(char) -> bool)
 -> Option<&'s str> {
    match parser.peek() {
        Some(&(start, first_character)) if !until(first_character) => {
            let mut end = start;
            parser.next();
            while let Some(&(idx, character)) = parser.peek() {
                end = match character {
                    ch if until(ch) => break,
                    _ => {
                        parser.next();
                        idx
                    }
                };
            }
            Some(&unsafe { source.get_unchecked(start..=end) })
        }
        Some(&(start, _)) => {
            Some(&unsafe { source.get_unchecked(start..start) })
        }
        None => None
    }

 }

#[inline]
#[allow(dead_code)]
pub fn consume_word<'p, 's: 'p>
(source: &'s str, parser: &'p mut Peekable<CharIndices>)
 -> Option<&'s str> {
    consume(source, parser, char::is_whitespace)
}

#[inline]
#[allow(dead_code)]
pub fn consume_parse<'p, 's: 'p, Out: FromStr>
(source: &'s str, parser: &'p mut Peekable<CharIndices>, until: fn(char) -> bool)
-> Result<Out, <Out as FromStr>::Err> {
    consume(source, parser, until).unwrap_or("").parse()
}

#[inline]
#[allow(dead_code)]
pub fn skip(parser: &mut Peekable<CharIndices>, until: fn(char) -> bool) {
    while let Some(&(_, character)) = parser.peek() {
        match character {
            ch if until(ch) => break,
            _ => { parser.next(); }
        }
    }
}

#[inline]
#[allow(dead_code)]
pub fn skip_whitespace(parser: &mut Peekable<CharIndices>) {
    skip(parser, |character| !character.is_whitespace())
}