.PHONY: format run test

format:
	@cargo fmt

run:
	@cargo run --quiet

test:
	@cargo test --quiet
