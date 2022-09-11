debug:
	cargo build

release:
	cargo build --release

install:
	cp ./target/release/nbu-rs /usr/local/bin/nbu