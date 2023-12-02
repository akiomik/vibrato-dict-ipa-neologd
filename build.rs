use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let mut output = File::create("tmp/mecab-ipadic-neologd.dic.zst")?;

    let mut f0 = File::open("dict/mecab-ipadic-neologd.dic.zst.aa")?;
    let mut f1 = File::open("dict/mecab-ipadic-neologd.dic.zst.ab")?;

    io::copy(&mut f0, &mut output)?;
    io::copy(&mut f1, &mut output)?;

    println!("cargo:rerun-if-changed=dict");

    Ok(())
}
