# git-bump [![CircleCI](https://circleci.com/gh/buckhx/git-bump.svg?style=svg)](https://circleci.com/gh/buckhx/git-bump)

Bumps a git project version using [semver](http://semver.org/) semantics.

The only _currently_ supported tag format is v.{MAJOR}.{MINOR}.{PATCH}

## Installation

For 64bit OSX & Linux, run the following commaned to download the correct binary and put it in your path. To upgrade, run this same script and it will replace the old binary.

    curl -sSL https://raw.githubusercontent.com/buckhx/git-bump/master/bin/install.py | python - /usr/local/bin

For other platforms see [Building](#building) and move the git-bump binary to somewhere on your PATH (/usr/local/bin)
    
## Usage

Just type git bump and the next patch version will be tagged (v0.2.2 -> v0.2.3).

```
$ git bump -h
git-bump 0.0.7
Bumps git version with semver semantics

USAGE:
    git-bump [FLAGS] [OPTIONS]

FLAGS:
    -n, --dry-run      Print result only, do not create tag
    -h, --help         Prints help information
        --init         Sets version to v0.0.1
        --major        Bumps MAJOR version, resets MINOR and PATCH
        --minor        Bumps MINOR version, resets PATCH
        --patch        Bumps PATCH [default]
        --uninstall    Removes git-bump from the system
    -V, --version      Prints version information

OPTIONS:
    -C <path>        Run as if git was started in <path>
```

## Building

This project is currently written in rust, so the rust platform is required. The only external dependency outside of the core rust platform is cargo-bump which sets the cargo version to the current tag appropriately. Here's an example of  

```
git clone https://github.com/buckhx/git-bump && cd git-bump  # clone
curl https://sh.rustup.rs -sSf | sh -s -- -y                 # Install rust
cargo install cargo-bump -q || true                          # Install cargo-bump
make build                                                   # generate debug build at target/debug/git-bump
```
