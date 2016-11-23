VERSION=$(shell git describe --always --tags)
TARGET="x86_64-apple-darwin"

build:
	cargo bump $(shell grep -oh -E "[^v]+" <<< ${VERSION})
	cargo build

test:
	cargo test

release:
	cargo build --release
	@git describe --tags --exact-match
	@cp target/release/git-bump target/release/git-bump_${TARGET}
	cargo bump $(shell grep -oh -E "[^v]+" <<< ${VERSION})
	ghr -u buckhx -r git-bump ${VERSION} target/release/git-bump_${TARGET}

version:
	@echo ${VERSION}
