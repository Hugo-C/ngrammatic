This crate provides fuzzy search/string matching using N-grams.

This implementation is character-based, rather than word based,
matching solely based on string similarity.

Licensed under the MIT license.


### Documentation

Not published yet.


### Installation

This crate is not yet published on [crates.io](https://crates.io/crates/).

To use it, add this to your Cargo.toml:

```toml
[dependencies]
ngrammatic = { git = 'https://github.com/compenguy/ngrammatic' }
```

To do fuzzy matching, build up your corpus of valid symbols like this:

```rust
use ngrammatic::{Corpus, Pad};

let mut corpus = Corpus::new(2, Pad::Auto, Pad::Auto);

// Build up the list of known words
corpus.add_text("pie");
corpus.add_text("animal");
corpus.add_text("tomato");
corpus.add_text("seven");
corpus.add_text("carbon");

// Now we can try an unknown/misspelled word, and find a similar match
// in the corpus
let word = String::from("tomacco");
if let Some(top_result) = corpus.search(word, 0.25).first() {
    if top_result.similarity > 0.99 {
        println!("✔ {}", top_result.text);
    } else {
        println!("❓{} (did you mean {}? [{:.0}% match])",
                 word,
                 top_result.text,
                 top_result.similarity * 100.0);
    }
} else {
    println!("🗙 {}", word);
}
```

### `case_insensitive_ngrams` feature

This crate has a feature, `case_insensitive_ngrams`, that can be enabled. It
folds all ngrams down to a single case, make all searches case insensitive.

Do not rely on this feature, as its likely to be replaced by a general-purpose
key-transformation closure in the future.
