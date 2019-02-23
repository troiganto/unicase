#![feature(test)]

extern crate test;
extern crate unicase;

mod bulgarian;
mod english;

use test::black_box;
use test::Bencher;
use unicase::Ascii;
use unicase::UniCase;

fn bench_make<F, T>(bencher: &mut Bencher, sample: &[&'static str], constructor: F)
where
    F: Fn(&'static str) -> T,
{
    bencher.iter(|| {
        for word in sample {
            let _ = black_box(constructor(word));
        }
    })
}

#[bench]
fn english_make_ascii(bencher: &mut Bencher) {
    bench_make(bencher, english::WORDS, Ascii::new)
}

#[bench]
fn english_make_unicase_new(bencher: &mut Bencher) {
    bench_make(bencher, english::WORDS, UniCase::new)
}

#[bench]
fn english_make_unicase_unicode(bencher: &mut Bencher) {
    bench_make(bencher, english::WORDS, UniCase::unicode)
}

#[bench]
fn bulgarian_make_ascii(bencher: &mut Bencher) {
    bench_make(bencher, bulgarian::WORDS, Ascii::new)
}

#[bench]
fn bulgarian_make_unicase_new(bencher: &mut Bencher) {
    bench_make(bencher, bulgarian::WORDS, UniCase::new)
}

#[bench]
fn bulgarian_make_unicase_unicode(bencher: &mut Bencher) {
    bench_make(bencher, bulgarian::WORDS, UniCase::unicode)
}

fn bench_eq_unicase<F>(bencher: &mut Bencher, sample: &[&'static str], constructor: F)
where
    F: Fn(&'static str) -> UniCase<&'static str>,
{
    let sample = sample.iter().cloned().map(constructor).collect::<Vec<_>>();
    bencher.iter(|| {
        sample
            .iter()
            .zip(sample.iter())
            .filter(|(left, right)| left == right)
            .count()
    })
}

#[bench]
fn english_eq_unicase_new(bencher: &mut Bencher) {
    bench_eq_unicase(bencher, english::WORDS, UniCase::new)
}

#[bench]
fn english_eq_unicase_unicode(bencher: &mut Bencher) {
    bench_eq_unicase(bencher, english::WORDS, UniCase::unicode)
}

#[bench]
fn bulgarian_eq_unicase_new(bencher: &mut Bencher) {
    bench_eq_unicase(bencher, bulgarian::WORDS, UniCase::new)
}

#[bench]
fn bulgarian_eq_unicase_unicode(bencher: &mut Bencher) {
    bench_eq_unicase(bencher, bulgarian::WORDS, UniCase::unicode)
}

fn bench_hash_unicase<F>(bencher: &mut Bencher, sample: &[&'static str], constructor: F)
where
    F: Fn(&'static str) -> UniCase<&'static str>,
{
    use std::collections::HashSet;
    let sample = sample
        .iter()
        .cloned()
        .map(constructor)
        .collect::<HashSet<_>>();
    bencher.iter(|| {
        sample
            .iter()
            .filter(|word| sample.contains(black_box(word)))
            .count()
    })
}

#[bench]
fn english_hash_unicase_new(bencher: &mut Bencher) {
    bench_hash_unicase(bencher, english::WORDS, UniCase::new)
}

#[bench]
fn english_hash_unicase_unicode(bencher: &mut Bencher) {
    bench_hash_unicase(bencher, english::WORDS, UniCase::unicode)
}

#[bench]
fn bulgarian_hash_unicase_new(bencher: &mut Bencher) {
    bench_hash_unicase(bencher, bulgarian::WORDS, UniCase::new)
}

#[bench]
fn bulgarian_hash_unicase_unicode(bencher: &mut Bencher) {
    bench_hash_unicase(bencher, bulgarian::WORDS, UniCase::unicode)
}
