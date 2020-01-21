build:
	cargo build --release
	rm -rf dist
	mkdir dist
	cp ding.wav dist/ding.wav
	cp target/release/trusty-timer.exe dist/trusty-timer.exe
	cp blocklist.txt dist/blocklist.txt
	cp TrustyTimer.lnk dist/TrustyTimer.lnk
debug: 
	RUST_BACKTRACE=1 cargo run