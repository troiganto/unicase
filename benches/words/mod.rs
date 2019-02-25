//! Word lists used in these benchmarks.
//!
//! Each word list has been generated from a UNIX dictionary provided under
//! `/usr/share/dict/<language>` by picking 5000 words at random. The English
//! word list was picked as the common use case of "mostly ASCII, but not
//! always". The Bulgarian word list was picked as the use case "no ASCII at
//! all, each word requires Unicode."

mod bulgarian;
mod english;

pub use self::bulgarian::WORDS as BULGARIAN;
pub use self::english::WORDS as ENGLISH;
