all: build

build:
	cd client/ && trunk build --release
	rm -rf docs/ && mkdir -p docs/ && mv client/dist/* docs/
	cargo build -p server --release
