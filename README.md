# `mutagen` applied

This repository demonstrates the application of `mutagen` to some large crates. The purpose of this repository is to find errors in `mutagen` by feeding it industry-sized inputs.

The `#[mutate]` attributes were added in a semi-automated way without detailed knowledge about each of the libraries. Therefore, the quality cannot be assessed professionally. Conclusions from the generated reports can only be drawn by programmers familiar with the code bases of the respective projects.

## How `mutagen` is applied to a crate

1. pull the crate and delete its `.git` folder. This project contains snapshots of crates
2. if edition is not 2018, run `cargo fix --edition` and add `edition = "2018"` to the `Cargo.toml`
3. run `scripts/apply-mutagen.sh <path>`

## Crates included in this repository

* rust-csv
* actix & actix-web
* indexmap
* hashbrown
* hyper


