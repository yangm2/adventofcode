# [2024 Advent of Code](https://adventofcode.com/2024)

## Cheatsheet

* use a [virtual Cargo workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html) to centrally manage individual crates
* testing out [2024 edition](https://doc.rust-lang.org/edition-guide/rust-2024/index.html) with [rust-toolchain.toml](./rust-toolchain.toml) ([ref](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file))

* initialize new year workspace ...
  ```sh
  # from 2024 directory
  % cargo init --name aoc2024
  ```
  then rewrite [Cargo.toml](./Cargo.toml) as a virtual workspace

* create a new day ...
  ```sh
  # from 2024 directory
  % cargo new --bin day1
  ```
  new members will inherit `workspace.package` settings from [the workspace Cargo.toml](./Cargo.toml)

* run all the days ...
  ```sh
  # from 2024/ directory
  % cargo run
  ```
