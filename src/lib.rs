use std::io::prelude::*;

pub fn read_dict() -> Result<vibrato::Dictionary, Box<dyn std::error::Error>> {
    let data0 = include_bytes!("../dict/mecab-ipadic-neologd.dic.zst.aa").as_slice();
    let data1 = include_bytes!("../dict/mecab-ipadic-neologd.dic.zst.ab").as_slice();
    let data = [data0, data1].concat();
    let mut decoder = ruzstd::StreamingDecoder::new(data.as_slice())?;
    let mut buff = vec![];
    decoder.read_to_end(&mut buff)?;
    vibrato::Dictionary::read(buff.as_slice()).map_err(|e| e.into())
}
