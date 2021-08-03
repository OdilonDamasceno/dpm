all:
	cargo build --release

install:
	install -D target/release/dpm /usr/bin/

uninstall:
	rm /usr/bin/dpm

clean:
	rm -rf target/