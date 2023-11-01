CARGO = cargo


.PHONY: run build publish


run:
	$(CARGO) run

build:
	$(CARGO) build --release

publish:
	$(CARGO) publish --registry crates-io
