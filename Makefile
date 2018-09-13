build:
	cargo build --package=nova-game --release
	mkdir -p release/resources
	cp target/release/nova-game release
	strip release/nova-game
	cp -r game/resources/* release/resources/
