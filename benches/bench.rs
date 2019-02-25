#[macro_use]
extern crate criterion;
extern crate unicase;

mod words;

use criterion::black_box;
use criterion::Bencher;
use criterion::Criterion;
use criterion::ParameterizedBenchmark;
use std::collections::HashSet;
use unicase::Ascii;
use unicase::UniCase;

/// Allows us to dynamically choose the `UniCase` constructor and whether to
/// upper-case a word or not.
#[derive(Debug, Clone, Copy)]
enum Transform {
    /// Use `UniCase::new()`.
    New,
    /// Use `UniCase::unicode()`.
    Unicode,
    /// Use `UniCase::new()` and upper-case the word.
    NewUpper,
    /// Use `UniCase::unicode()` and upper-case the word.
    UnicodeUpper,
}

impl Transform {
    /// Get a `UniCase` from a string slice.
    fn transform(self, word: &str) -> UniCase<String> {
        match self {
            Transform::New => UniCase::new(word.to_owned()),
            Transform::Unicode => UniCase::unicode(word.to_owned()),
            Transform::NewUpper => UniCase::new(word.to_uppercase()),
            Transform::UnicodeUpper => UniCase::unicode(word.to_uppercase()),
        }
    }
}

/// Newtype wrapper that provides a better `Debug` impl for our word lists.
/// This avoids `criterion` putting the entire lists of 5000 words each into
/// its report. Instead it just shows "English" or "Bulgarian" or whatever
/// `name` is.
#[derive(Clone)]
pub struct LangItem<T> {
    pub name: &'static str,
    pub inner: T,
}

impl<T> std::fmt::Debug for LangItem<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

/// The two word lists used in our benchmarks.
static WORD_LISTS: [LangItem<&'static [&'static str]>; 2] = [
    LangItem {
        name: "English",
        inner: words::ENGLISH,
    },
    LangItem {
        name: "Bulgarian",
        inner: words::BULGARIAN,
    },
];

/// Benchmark the type constructors provided by `unicase`.
fn constructor(c: &mut Criterion) {
    // The repetitive parts of our benchmark. We use a macro instead of a
    // function in order to guarantee inlining.
    macro_rules! benchmark {
        ($item:expr, $func:expr) => {
            $item.inner.iter().cloned().map($func).for_each(|s| {
                let _ = black_box(s);
            })
        };
    }
    c.bench(
        "Constructor",
        ParameterizedBenchmark::new(
            "Ascii::new",
            |bencher, item| bencher.iter(|| benchmark!(item, Ascii::new)),
            &WORD_LISTS,
        )
        .with_function("UniCase::new", |bencher, item| {
            bencher.iter(|| benchmark!(item, UniCase::new))
        })
        .with_function("UniCase::unicode", |bencher, item| {
            bencher.iter(|| benchmark!(item, UniCase::unicode))
        }),
    );
}

/// Benchmark case-insensitive string comparison via `==`.
fn comparison(c: &mut Criterion) {
    // The repetitive parts of our benchmark. It's too complicated for a macro,
    // so instead we use `inline(always)` and hope it really gets inlined.
    #[inline(always)]
    fn benchmark(bencher: &mut Bencher, words: &[&'static str], t: Transform) {
        let left: Vec<_> = words.iter().map(|s| t.transform(s)).collect();
        let right = {
            let mut vec: Vec<_> = words.iter().map(|s| t.transform(s)).collect();
            let len = vec.len();
            // Put every other element into the wrong position so that
            // comparison returns both `true` and `false`.
            for i in (0..len / 2).step_by(2) {
                vec.swap(i, len - i - 1);
            }
            vec
        };
        bencher.iter(|| {
            left.iter()
                .zip(right.iter())
                .filter(|&(lword, rword)| lword == rword)
                .count()
        })
    }
    c.bench(
        "Comparison",
        ParameterizedBenchmark::new(
            "UniCase::new/lower-vs-lower",
            |bencher, item| benchmark(bencher, item.inner, Transform::New),
            &WORD_LISTS,
        )
        .with_function("UniCase::new/lower-vs-upper", |bencher, item| {
            benchmark(bencher, item.inner, Transform::NewUpper)
        })
        .with_function("UniCase::unicode/lower-vs-lower", |bencher, item| {
            benchmark(bencher, item.inner, Transform::Unicode)
        })
        .with_function("UniCase::unicode/lower-vs-upper", |bencher, item| {
            benchmark(bencher, item.inner, Transform::UnicodeUpper)
        }),
    );
}

/// Benchmark hashing of case-insensitive strings.
fn lookup(c: &mut Criterion) {
    #[inline(always)]
    fn benchmark(bencher: &mut Bencher, words: &[&'static str], t: Transform) {
        // Mistune the step so that `contains()` returns both `true` and
        // `false`.
        let left: HashSet<_> = words.iter().step_by(2).map(|s| t.transform(s)).collect();
        let right: Vec<_> = words.iter().step_by(3).map(|s| t.transform(s)).collect();
        bencher.iter(|| {
            right
                .iter()
                .map(|word| left.contains(black_box(word)))
                .count()
        })
    }
    c.bench(
        "Lookup",
        ParameterizedBenchmark::new(
            "UniCase::new/lower-vs-lower",
            |bencher, item| benchmark(bencher, item.inner, Transform::New),
            &WORD_LISTS,
        )
        .with_function("UniCase::new/lower-vs-upper", |bencher, item| {
            benchmark(bencher, item.inner, Transform::NewUpper)
        })
        .with_function("UniCase::unicode/lower-vs-lower", |bencher, item| {
            benchmark(bencher, item.inner, Transform::Unicode)
        })
        .with_function("UniCase::unicode/lower-vs-upper", |bencher, item| {
            benchmark(bencher, item.inner, Transform::UnicodeUpper)
        }),
    );
}

criterion_group!(benches, constructor, comparison, lookup);
criterion_main!(benches);
