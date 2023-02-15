
# Need to install 'cargo-expand' to use 'cargo expand'
#
#   $ cargo install cargo-expand

.PHONY: clean expand run

EXPANDED = expanded.rs

clean:
	cargo clean
	rm $(EXPANDED)

expand:
	cargo expand > $(EXPANDED)

run:
	cargo run -q
