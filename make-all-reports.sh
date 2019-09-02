#!/bin/bash

# prepare and update runner setup
git submodule update --init
mkdir -p reports
cargo install --path mutagen/mutagen-runner --root . --force --offline

crates=(actix actix-web chrono indexmap rust-csv)

for crate in ${crates[@]}; do
  echo $crate
  pushd $crate
  ../bin/cargo-mutagen | tee ../reports/$crate.report
  popd
done
