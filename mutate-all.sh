#!/bin/bash

set -x

mkdir -p reports

for crate in $(ls); do
  pushd $crate
  cargo mutagen | tee ../reports/$crate.report
  popd
done
