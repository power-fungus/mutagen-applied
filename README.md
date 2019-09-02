# `mutagen` applied

This repository demonstrates the application of `mutagen` to some large crates.

The purpose of this repository is to find errors in `mutagen` by feeding it industry-sized inputs.

## How `mutagen` is applied to a crate
* pull the crate and delete its `.git` folder
* if edition is not 2018, run `cargo fix --edition` and add `edition = "2018"` to the `Cargo.toml`
* add dependency `mutagen`
* enable mutations for all files in `src` folder: `fd '\.rs' src | xargs sed -i -e 's/^impl/#[cfg_attr(test, ::mutagen::mutate)] impl/g;s/^fn/#[cfg_attr(test, ::mutagen::mutate)] fn/g'`

## Crates included in this repository

* chrono
* rust-csv
* actix & actix-web
* indexmap

