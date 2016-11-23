# git-bump [![CircleCI](https://circleci.com/gh/buckhx/git-bump.svg?style=svg)](https://circleci.com/gh/buckhx/git-bump)

Bumps a git project version using semver semantics

## Installation

Run the following commaned to download the right binary and put it in your path. Install script only works on OSX for now.

    curl -sSL https://raw.githubusercontent.com/buckhx/git-bump/master/bin/install.py | sudo python - /usr/local/bin
    
## Usage

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
