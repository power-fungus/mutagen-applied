#!/bin/bash

set -e

# prepare and update runner setup
git submodule update --init
mkdir -p reports
cargo install --path mutagen/mutagen-runner --root . --force --offline

crates=$(ls crates)

cd crates

for crate in ${crates[@]}; do
  echo $crate
  pushd $crate
  cargo test --no-run
  popd
done

for crate in ${crates[@]}; do
  echo $crate
  pushd $crate
  cargo test --tests
  popd
done

for crate in ${crates[@]}; do
  echo $crate
  pushd $crate
  ../../bin/cargo-mutagen | tee ../../reports/$crate.report
  popd
done
