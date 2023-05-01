#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly SSH_HOST=$1
readonly TARGET_PATH=$2
readonly TARGET_ARCH=aarch64-unknown-linux-gnu
readonly SOURCE_PATH=./target/${TARGET_ARCH}/debug/garden-pi
cargo build --release --target=${TARGET_ARCH}
rsync ${SOURCE_PATH} ${SSH_HOST}:${TARGET_PATH}
ssh -t ${SSH_HOST} ${TARGET_PATH}