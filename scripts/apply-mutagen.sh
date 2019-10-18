#!/bin/sh

# experimental script to try to apply mutagen automatically
# only applies to edition-2018 crates

set -e

mutagenpath=$(realpath --relative-to="$(pwd)/$1" $(pwd)/mutagen/mutagen)

cd $1

if [[ ! -f Cargo.toml ]]; then
  echo "path does not contain a cargo crate"
  exit 1
fi

# check edition
if grep -q -r 'edition.*"2018"' Cargo.toml; then
  echo "correct edition"
else
  echo "incorrect edition"
  exit 1
fi

# add mutagen dependency
if grep -q -r '^\[dev-dependencies\]$' Cargo.toml; then
  if ! grep -q '^mutagen = ' Cargo.toml; then
    sed -i -e "/^\[dev-dependencies\]/ a mutagen = {path = \"$mutagenpath\"}" Cargo.toml
    echo "dependency mutagen added"
  else
    echo "dependency present"
  fi
else
  echo "no [dev-dependencies] section"
  exit 1
fi

find src/ -name '*.rs' | xargs sed -i -re 's/^(impl|fn|pub fn)/#[cfg_attr(test, ::mutagen::mutate)] \1/g'
echo "attribute #[mutate] added"


