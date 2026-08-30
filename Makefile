.PHONY: build run package clean

build:
	cd src-tauri && cargo build --release

run:
	cd src-tauri && cargo tauri dev

package: build
	bash build-luppo.sh

clean:
	cd src-tauri && cargo clean
