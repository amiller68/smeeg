#!/usr/bin/env bash

set -o errexit
set -o nounset

OLLAMA_CONTAINER_NAME="smeeg-ollama"
OLLAMA_VOLUME_NAME="smeeg-ollama-data"

MODEL_REPO="NousResearch/Hermes-2-Pro-Mistral-7B-GGUF"
MODEL_FILE="Hermes-2-Pro-Mistral-7B.Q4_K_M.gguf"
MODEL_DIR="./data/"
MODEL_NAME="nous-hermes-2-pro"

OLLAMA_SERVER_URL="http://localhost:11434"

CONTAINER_RUNTIME="podman"
if which docker &>/dev/null; then
	CONTAINER_RUNTIME="docker"
fi

function model-name {
	echo ${MODEL_NAME}
}

# TODO: I should really be executhing these int the container, but for now this is ok
function create-model {
	pull-model

	# Check if the model already exists
	if ollama ls | grep ${MODEL_NAME} &>/dev/null; then
		# If FORCE either not SET and SET TO false, then return
		if [ -z ${FORCE+x} ] || [ ${FORCE} == "false" ]; then
			return
		fi
	fi

	ollama create ${MODEL_NAME}
}

# TODO: document this
function pull-model {
	# Check of the model is already downloaded
	if [ -f ${MODEL_DIR}/${MODEL_FILE} ]; then
		return
	fi

	huggingface-cli download ${MODEL_REPO} ${MODEL_FILE} --local-dir ${MODEL_DIR} --local-dir-use-symlinks False
}

function server-url {
	echo ${OLLAMA_SERVER_URL}
}

function run {
	start-ollama-container
}

# Helpers:

function start-ollama-container {
	ensure-ollama-container-exists
	${CONTAINER_RUNTIME} start ${OLLAMA_CONTAINER_NAME}
}

function ensure-ollama-container-exists {
	docker pull ollama/ollama
	create-ollama-container
}

function create-ollama-container {
	if ${CONTAINER_RUNTIME} ps -a | grep ${OLLAMA_CONTAINER_NAME} &>/dev/null; then
		return
	fi

	${CONTAINER_RUNTIME} volume create ${OLLAMA_VOLUME_NAME} || true

	${CONTAINER_RUNTIME} run \
		--name ${OLLAMA_CONTAINER_NAME} \
		--publish 11434:11434 \
		--volume ${OLLAMA_VOLUME_NAME}:/root/.ollama \
		--detach \
		ollama/ollama
}

function clean() {
	docker stop ${OLLAMA_CONTAINER_NAME} || true
	${CONTAINER_RUNTIME} rm -fv ${OLLAMA_CONTAINER_NAME} || true
	${CONTAINER_RUNTIME} volume rm -f ${OLLAMA_VOLUME_NAME} || true
}

$1
