PREFIX := /usr/bin
FILE := example.vcl
I := 4

target/release/vcl-formatter: src/*.rs Cargo.toml Cargo.lock
	cargo build --release

build: target/release/vcl-formatter
.PHONY: build

$(PREFIX)/vcl-formatter: target/release/vcl-formatter
	install -m 755 target/release/vcl-formatter "$(PREFIX)/"

install: $(PREFIX)/vcl-formatter
.PHONY: install

uninstall:
	rm -f "$(PREFIX)/vcl-formatter"
.PHONY: uninstall

bench: target/release/vcl-formatter
	hyperfine -N --warmup 5 './target/release/vcl-formatter example.vcl'
.PHONY: bench

diff:
	cargo run -- -i "$(I)" "$(FILE)" | diff -u --color "$(FILE)" -
.PHONY: diff
