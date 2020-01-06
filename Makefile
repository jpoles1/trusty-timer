build:
	cargo build --release
	rm -rf dist
	mkdir dist
	cp ding.wav dist/ding.wav
	cp target/release/trusty-timer.exe dist/trusty-timer.exe