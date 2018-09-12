build:
	cargo build --release
	mkdir -p release
	cp target/release/nova-game release
	strip release/nova-game
	cp -r game/resources release/resources
