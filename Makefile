PREFIX := /usr/bin

vendor/varnishls/vendor/tree-sitter-vcl/bindings:
	rm -rf vendor
	mkdir -p vendor
	cd vendor && git clone -q --depth 1 --branch "v0.0.10" https://github.com/M4R7iNP/varnishls.git
	cd vendor/varnishls && make tree-sitter-vcl

target/release/vcl-formatter: vendor/varnishls/vendor/tree-sitter-vcl/bindings src/*.rs Cargo.toml Cargo.lock
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
	seq 0 3 | sed 's|.*|"./target/release/vcl-formatter -t \0 -p example.vcl"|' | xargs hyperfine -Ni
.PHONY: bench
