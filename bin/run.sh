#!/usr/bin/env bash

set -o errexit

export OLLAMA_SERVER_URL=$(bin/ollama.sh server-url)
export OLLAMA_MODEL_NAME=$(bin/ollama.sh model-name)
export CHROMA_DATABASE_URL=$(bin/chroma.sh database-url)
export SQLITE_DATABASE_URL=$(bin/sqlite.sh database-url)
make ollama
make chroma
make sqlite

cargo run
