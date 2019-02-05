build: clean
	@echo Building release…
	@cargo build --package=tvb --release

	@mkdir -p release/assets
	@cp target/release/tvb release

	@echo Stripping binary…
	@strip release/tvb

clean:
	@rm -rf release
