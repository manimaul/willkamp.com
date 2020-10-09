#!/usr/bin/env bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
SRC_DIR="${DIR}/src"
BUILD_DIR="${DIR}/static"

pushd ${DIR}

echo "watching ${SRC_DIR}"
fswatch -o ${SRC_DIR} | while read num; do
  echo "building"
  wasm-pack build --target web --out-name wasm --out-dir ${BUILD_DIR} || echo "failed"
done

popd