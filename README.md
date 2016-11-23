# git-bump [![CircleCI](https://circleci.com/gh/buckhx/git-bump.svg?style=svg)](https://circleci.com/gh/buckhx/git-bump)

Bumps a git project version using [semver](http://semver.org/) semantics.

The only _currently_ supported tag format is v.{MAJOR}.{MINOR}.{PATCH}

## Installation

For OSX, run the following commaned to download the correct binary and put it in your path.

    curl -sSL https://raw.githubusercontent.com/buckhx/git-bump/master/bin/install.py | sudo python - /usr/local/bin

For Linux, download the latest unknown-linux-gnu binary in [releases](https://github.com/buckhx/git-bump/releases), add executable permissions (chmod +x), rename the binary to git-bump and move it somewhere on your PATH (/usr/local/bin)
    
## Usage

Just type git bump and the next patch version will be tagged (v0.2.2 -> v0.2.3).

```
$ git bump --help
git-bump 0.0.3
Bumps git version with semver semantics

USAGE:
    git-bump [FLAGS]

FLAGS:
    -n, --dry-run    Print result only, do not create tag
    -h, --help       Prints help information
        --init       Sets version to v0.0.1
        --major      Bumps MAJOR version, resets MINOR and PATCH
        --minor      Bumps MINOR version, resets PATCH
        --patch      Bumps PATCH [default]
    -V, --version    Prints version information
```

## Building

This project is currently written in rust, so the rust platform is required. The only external dependency outside of the core rust platform is cargo-bump which sets the cargo version to the current tag appropriately. Here's an example of  

```
git clone https://github.com/buckhx/git-bump && cd git-bump  # clone
curl https://sh.rustup.rs -sSf | sh -s -- -y                 # Install rust
cargo install cargo-bump -q || true                          # Install cargo-bump
make build                                                   # generate debug build at target/debug/git-bump
```
