VERSION=$(shell git describe --always --tags --dirty)
TARGET="x86_64-apple-darwin"

build:
	cargo build

release:
	cargo build --release
	git describe --exact-match
	cp target/release/git-bump target/release/git-bump_${TARGET}
	ghr -u buckhx -r git-bump ${VERSION} target/release/git-bump_${TARGET}

version:
	@echo ${VERSION}
