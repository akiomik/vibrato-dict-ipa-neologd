//! # vibrato-dict-ipadic-neologd
//!
//! A compiled mecab-ipadic-neologd dictionary for vibrato.

#![deny(warnings, missing_docs)]

/// Error types
pub mod error;

use std::io::prelude::*;

/// Reads mecab-ipadic-neologd as [vibrato::Dictionary]
///
/// # Examples
///
/// ```
/// let dict = vibrato_dict_ipa_neologd::read_dict().unwrap();
/// let tokenizer = vibrato::Tokenizer::new(dict);
/// let mut worker = tokenizer.new_worker();
/// worker.reset_sentence("本とカレーの街神保町へようこそ。");
/// worker.tokenize();
///
/// assert_eq!(9, worker.num_tokens());
/// assert_eq!("本", worker.token(0).surface());
/// assert_eq!("と", worker.token(1).surface());
/// assert_eq!("カレー", worker.token(2).surface());
/// assert_eq!("の", worker.token(3).surface());
/// assert_eq!("街", worker.token(4).surface());
/// assert_eq!("神保町", worker.token(5).surface());
/// assert_eq!("へ", worker.token(6).surface());
/// assert_eq!("ようこそ", worker.token(7).surface());
/// assert_eq!("。", worker.token(8).surface());
/// ```
pub fn read_dict() -> Result<vibrato::Dictionary, error::ReadError> {
    let data = include_bytes!("../tmp/mecab-ipadic-neologd.dic.zst");
    let mut decoder = ruzstd::StreamingDecoder::new(data.as_slice())?;
    let mut buff = vec![];
    decoder.read_to_end(&mut buff)?;
    let dict = vibrato::Dictionary::read(buff.as_slice())?;
    Ok(dict)
}
