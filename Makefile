vendor/varnishls:
	mkdir -p vendor
	cd vendor && git clone https://github.com/M4R7iNP/varnishls.git

vendor/varnishls/vendor/tree-sitter-vcl/bindings: vendor/varnishls
	cd "$<" && make tree-sitter-vcl

deps: vendor/varnishls/vendor/tree-sitter-vcl/bindings
.PHONY: deps
