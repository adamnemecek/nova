build: clean
	@echo Building release…
	@cargo build --package=tvb --release

	@mkdir -p release
	@cp -r target/release/tvb assets release

	@echo Stripping binary…
	@strip release/tvb

clean:
	@rm -rf release

test:
	@cd nova && cargo test --all
	@cargo test --all

.PHONY: build clean test
