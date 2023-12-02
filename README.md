# vibrato-dict-ipa-neologd

[![CI](https://github.com/akiomik/vibrato-dict-ipa-neologd/actions/workflows/ci.yml/badge.svg)](https://github.com/akiomik/vibrato-dict-ipa-neologd/actions/workflows/ci.yml)

A compiled [mecab-ipadic-neologd](https://github.com/neologd/mecab-ipadic-neologd) dictionary for [vibrato](https://github.com/daac-tools/vibrato).

## Install

> [!IMPORTANT]
> Due to file size limitations, this library does not support installation from crates.io.

Add the following line to your `Cargo.toml`:

```toml
[dependencies]
vibrato-dict-ipa-neologd = { git = "https://github.com/akiomik/vibrato-dict-ipa-neologd" }
```

## Usage

```rust
let dict = vibrato_dict_ipa_neologd::read_dict().unwrap();
let tokenizer = vibrato::Tokenizer::new(dict);
let mut worker = tokenizer.new_worker();
worker.reset_sentence("本とカレーの街神保町へようこそ。");
worker.tokenize();

assert_eq!(9, worker.num_tokens());
assert_eq!("本", worker.token(0).surface());
assert_eq!("と", worker.token(1).surface());
assert_eq!("カレー", worker.token(2).surface());
assert_eq!("の", worker.token(3).surface());
assert_eq!("街", worker.token(4).surface());
assert_eq!("神保町", worker.token(5).surface());
assert_eq!("へ", worker.token(6).surface());
assert_eq!("ようこそ", worker.token(7).surface());
assert_eq!("。", worker.token(8).surface());
```
