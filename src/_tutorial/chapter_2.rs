//! # Chapter 2: Tokens and Tags
//!
//! The simplest *useful* parser you can write is one which matches tokens.
//!
//! ## Tokens
//!
//! Matching a single token literal is common enough that `Parser` is implemented for
//! `char`.
//!
//! ```rust
//! # use winnow::Parser;
//! # use winnow::PResult;
//! #
//! fn parse_prefix(input: &mut &str) -> PResult<char> {
//!     '0'.parse_next(input)
//! }
//!
//! fn main()  {
//!     let mut input = "0x1a2b Hello";
//!
//!     let output = parse_prefix.parse_next(&mut input).unwrap();
//!
//!     assert_eq!(input, "x1a2b Hello");
//!     assert_eq!(output, '0');
//!
//!     assert!(parse_prefix.parse_next(&mut "d").is_err());
//! }
//! ```
//!
//! ## Tags
//!
//! One of the most frequent way of matching a token is when they are combined into a string.
//! Again, this is common enough that `Parser` is implemented for `&str`:
//!
//! ```rust
//! # use winnow::Parser;
//! # use winnow::PResult;
//! #
//! fn parse_prefix<'s>(input: &mut &'s str) -> PResult<&'s str> {
//!     "0x".parse_next(input)
//! }
//!
//! fn main()  {
//!     let mut input = "0x1a2b Hello";
//!
//!     let output = parse_prefix.parse_next(&mut input).unwrap();
//!     assert_eq!(input, "1a2b Hello");
//!     assert_eq!(output, "0x");
//!
//!     assert!(parse_prefix.parse_next(&mut "0o123").is_err());
//! }
//! ```
//!
//! In `winnow`, we call this type of parser a [`tag`].
//!
//! ## Character Classes
//!
//! Selecting a single `char` or a [`tag`] is fairly limited.  Sometimes, you will want to select one of several
//! `chars` of a specific class, like digits. For this, we use the [`one_of`] parer:
//!
//! ```rust
//! # use winnow::Parser;
//! # use winnow::PResult;
//! use winnow::token::one_of;
//!
//! fn parse_digits(input: &mut &str) -> PResult<char> {
//!     one_of(('0'..='9', 'a'..='f', 'A'..='F')).parse_next(input)
//! }
//!
//! fn main() {
//!     let mut input = "1a2b Hello";
//!
//!     let output = parse_digits.parse_next(&mut input).unwrap();
//!     assert_eq!(input, "a2b Hello");
//!     assert_eq!(output, '1');
//!
//!     assert!(parse_digits.parse_next(&mut "Z").is_err());
//! }
//! ```
//!
//! > **Aside:** [`one_of`] might look straightforward, a function returning a value that implements `Parser`.
//! > Let's look at it more closely as its used above (resolving all generic parameters):
//! > ```rust
//! > # use winnow::prelude::*;
//! > # use winnow::error::Error;
//! > pub fn one_of<'i>(
//! >     list: &'static [char]
//! > ) -> impl Parser<&'i str, char, Error<&'i str>> {
//! >     // ...
//! > #    winnow::token::one_of(list)
//! > }
//! > ```
//! > If you have not programmed in a language where functions are values, the type signature of the
//! > [`one_of`] function might be a surprise.
//! > The function [`one_of`] *returns a function*. The function it returns is a
//! > `Parser`, taking a `&str` and returning an `PResult`. This is a common pattern in winnow for
//! > configurable or stateful parsers.
//!
//! Some of character classes are common enough that a named parser is provided, like with:
//! - [`line_ending`][crate::ascii::line_ending]: Recognizes an end of line (both `\n` and `\r\n`)
//! - [`newline`][crate::ascii::newline]: Matches a newline character `\n`
//! - [`tab`][crate::ascii::tab]: Matches a tab character `\t`
//!
//! You can then capture sequences of these characters with parsers like [`take_while`].
//! ```rust
//! # use winnow::Parser;
//! # use winnow::PResult;
//! use winnow::token::take_while;
//!
//! fn parse_digits<'s>(input: &mut &'s str) -> PResult<&'s str> {
//!     take_while(1.., ('0'..='9', 'a'..='f', 'A'..='F')).parse_next(input)
//! }
//!
//! fn main() {
//!     let mut input = "1a2b Hello";
//!
//!     let output = parse_digits.parse_next(&mut input).unwrap();
//!     assert_eq!(input, " Hello");
//!     assert_eq!(output, "1a2b");
//!
//!     assert!(parse_digits.parse_next(&mut "Z").is_err());
//! }
//! ```
//!
//! We could simplify this further with by using one of the built-in character classes, [`hex_digit1`]:
//! ```rust
//! # use winnow::Parser;
//! # use winnow::PResult;
//! use winnow::ascii::hex_digit1;
//!
//! fn parse_digits<'s>(input: &mut &'s str) -> PResult<&'s str> {
//!     hex_digit1.parse_next(input)
//! }
//!
//! fn main() {
//!     let mut input = "1a2b Hello";
//!
//!     let output = parse_digits.parse_next(&mut input).unwrap();
//!     assert_eq!(input, " Hello");
//!     assert_eq!(output, "1a2b");
//!
//!     assert!(parse_digits.parse_next(&mut "Z").is_err());
//! }
//! ```

#![allow(unused_imports)]
use crate::ascii::hex_digit1;
use crate::stream::ContainsToken;
use crate::token::one_of;
use crate::token::tag;
use crate::token::take_while;
use crate::Parser;
use std::ops::RangeInclusive;

pub use super::chapter_1 as previous;
pub use super::chapter_3 as next;
