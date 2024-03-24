# NB: the project will be built if make is invoked without any arguments.
.PHONY: default
default: build

.PHONY: build
build:
	cargo build

.PHONY: check
check:
	cargo check

.PHONY: clean
clean:
	./bin/ollama.sh clean
	cargo clean

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: fmt-check
fmt-check:
	cargo fmt --all -- --check

.PHONY: clippy
clippy:
	cargo clippy --all-targets --all-features --tests -- -D warnings

.PHONY: sqlite
sqlite:
	./bin/sqlite.sh create && \
		./bin/sqlite.sh queries && \
			./bin/sqlite.sh migrate

.PHONY: ollama
ollama:
	./bin/ollama.sh run && \
		./bin/ollama.sh create-model

.PHONY: chroma
chroma:
	./bin/chroma.sh run

.PHONY: ollama-clean
ollama-clean:
	./bin/ollama.sh clean

.PHONY: test
test:
	cargo test --all --workspace --bins --tests --benches
