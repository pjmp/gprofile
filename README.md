# gprofile [![Crates.io](https://img.shields.io/crates/v/gprofile)](https://crates.io/crates/gprofile) ![License](https://img.shields.io/crates/l/gprofile)

> Quickly switch git user, email with ease.

## Installation

If you have rust toolchain installed, you can just do:

```shell
cargo install gprofile
```

Alternatively, you can download pre-build binaries from the [release page](https://github.com/pjmp/gprofile/releases).

## Usage

```text
USAGE:
  gprofile [FLAGS]
  gprofile [OPTIONS] <PROFILE>

FLAGS:
  -h, --help            Prints help information
  -v, --version         Prints version information

OPTIONS:
  -c, --create          Create a new profile
  -l, --list            List available profiles
  -d, --delete PROFILE  Delete a given profile
  -e, --edit PROFILE    Edit a given profile
  -u, --use PROFILE     Use profile as current git config.user & config.email
```
