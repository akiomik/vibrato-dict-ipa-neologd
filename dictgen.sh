#!/bin/bash

IPADIC_VERSION="2.7.0-20070801"
NEOLOGD_VERSION="20200910"
VIBRATO_VERSION="v0.5.1"

OUTPUT_DIR="./dict"
NEOLOGD_ROOT="./mecab-ipadic-neologd"
NEOLOGD_BUILD_DIR="$NEOLOGD_ROOT/build/mecab-ipadic-${IPADIC_VERSION}-neologd-${NEOLOGD_VERSION}"

# build mecab-ipadic-neologd
$NEOLOGD_ROOT/bin/install-mecab-ipadic-neologd -a -y

# copy seed files
TEMP_DIR=$(mktemp -d -t vibrato-dict-ipa-neologd)
cp -r $NEOLOGD_ROOT/seed $TEMP_DIR
unxz $TEMP_DIR/seed/*.xz

# copy system dictionary files
echo $TEMP_DIR
cat $NEOLOGD_BUILD_DIR/*.csv $TEMP_DIR/seed/*.csv > $TEMP_DIR/lex.csv
cp $NEOLOGD_BUILD_DIR/matrix.def $TEMP_DIR/matrix.def
cp $NEOLOGD_BUILD_DIR/unk.def $TEMP_DIR/unk.def
cp $NEOLOGD_BUILD_DIR/char.def $TEMP_DIR/char.def

# compile system dictionary
cargo install --git "https://github.com/daac-tools/vibrato" --tag "$VIBRATO_VERSION" --root $TEMP_DIR compile
$TEMP_DIR/bin/compile \
    -l $TEMP_DIR/lex.csv \
    -m $TEMP_DIR/matrix.def \
    -u $TEMP_DIR/unk.def \
    -c $TEMP_DIR/char.def \
    -o $TEMP_DIR/mecab-ipadic-neologd.dic.zst

# split system dictionary
split -b 100M $TEMP_DIR/mecab-ipadic-neologd.dic.zst mecab-ipadic-neologd.dic.zst.
mv $TEMP_DIR/mecab-ipadic-neologd.dic.zst.* $OUTPUT_DIR
