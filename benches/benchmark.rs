use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ngrammatic::{CorpusBuilder, Pad};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use rand_pcg::Pcg64;

fn search_benchmark(c: &mut Criterion) {
    let mut corpus = CorpusBuilder::new()
        .arity(2)
        .pad_full(Pad::Auto)
        .finish();

    // Build up the list of known words
    corpus.add_text("pie");
    corpus.add_text("animal");
    corpus.add_text("tomato");
    corpus.add_text("seven");
    corpus.add_text("carbon");
    corpus.add_text("some test");
    corpus.add_text("some other test");

    c.bench_function(
        "search",
        |b| {
            b.iter(|| {
                corpus.search("tomacco", black_box(0.25)).first();
            });
        });
}

fn rng_search_benchmark(c: &mut Criterion) {
    let mut corpus = CorpusBuilder::new()
        .arity(3)
        .pad_full(Pad::Auto)
        .finish();

    let seed = 42;
    let mut rng = Pcg64::seed_from_u64(seed);

    let nb_corpus = 100;
    for _ in 1..nb_corpus {
        let rand_string: String = (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        corpus.add_text(&rand_string.to_string());
    }

    let nb_search = 20;
    let mut words_to_search = Vec::new();
    for _ in 1..nb_search {
        let rand_string: String = (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        words_to_search.push(rand_string)
    }
    c.bench_function(
        "rng_search",
        |b| {
            b.iter(|| {
                for word in &words_to_search {
                    corpus.search(&word.to_string(), black_box(0.25)).first();
                }
            });
        });
}

fn contains_key_return_true_benchmark(c: &mut Criterion) {
    let mut corpus = CorpusBuilder::new()
        .arity(2)
        .pad_full(Pad::Auto)
        .finish();

    // Build up the list of known words
    corpus.add_text("pie");
    corpus.add_text("animal");
    corpus.add_text("tomato");
    corpus.add_text("seven");
    corpus.add_text("carbon");
    corpus.add_text("some test");
    corpus.add_text("some other test");

    c.bench_function(
        "contains key return true",
        |b| {
            b.iter(|| {
                corpus.key(black_box("tomato"));
            });
        });
}

fn contains_key_return_false_benchmark(c: &mut Criterion) {
    let mut corpus = CorpusBuilder::new()
        .arity(2)
        .pad_full(Pad::Auto)
        .finish();

    // Build up the list of known words
    corpus.add_text("pie");
    corpus.add_text("animal");
    corpus.add_text("tomato");
    corpus.add_text("seven");
    corpus.add_text("carbon");
    corpus.add_text("some test");
    corpus.add_text("some other test");

    c.bench_function(
        "contains key return false",
        |b| {
            b.iter(|| {
                corpus.key(black_box("tomacco"));
            });
        });
}


criterion_group!(benches, search_benchmark, rng_search_benchmark); //, contains_key_return_true_benchmark, contains_key_return_false_benchmark);
criterion_main!(benches);
