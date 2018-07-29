#!/usr/bin/env bash

set -e

SCRIPT_FILE=$(readlink -f ${0})
SCRIPT_PATH=$(dirname ${SCRIPT_FILE})

mkdir -p "${SCRIPT_PATH}/Release"
cd "${SCRIPT_PATH}/Release"
cmake -G Ninja \
  -D CMAKE_C_COMPILER=clang \
  -D CMAKE_CXX_COMPILER=clang++ \
  -D CMAKE_CXX_FLAGS="-stdlib=libc++ -fno-rtti -fcolor-diagnostics" \
  -D CMAKE_BUILD_TYPE=Release \
  -D BENCHMARK_DOWNLOAD_DEPENDENCIES:BOOL=ON \
  -D BENCHMARK_ENABLE_TESTING:BOOL=OFF \
  -D DBENCHMARK_ENABLE_GTEST_TESTS:BOOL=OFF \
  -D CMAKE_EXPORT_COMPILE_COMMANDS=1 ..
