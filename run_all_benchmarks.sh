#!/usr/bin/env bash

set -e

SCRIPT_DIR_RELATIVE=$(dirname "${0}")
SCRIPT_DIR_ABSOLUTE=$(realpath "${SCRIPT_DIR_RELATIVE}")

RUST_DIR="${SCRIPT_DIR_ABSOLUTE}/rust"
CPP_DIR="${SCRIPT_DIR_ABSOLUTE}/cpp"
DLANG_DIR="${SCRIPT_DIR_ABSOLUTE}/dlang"

function title() {
  printf -- "----- ${1} -----\n"
  date --utc
  printf "\n"
}

function cargo_with_features() {
  printf "CONFIGURATION: ${1}"
  cargo +nightly bench --quiet --features="${1}"
}

function rust_with_hash() {
  cargo_with_features "${1}"

  cargo_with_features "${1},reserve_hm"

  cargo_with_features "${1},string_key"
  cargo_with_features "${1},string_value"
  cargo_with_features "${1},string_key,string_value"

  cargo_with_features "${1},string_pad,string_key"
  cargo_with_features "${1},string_pad,string_value"
  cargo_with_features "${1},string_pad,string_key,string_value"
}

function benchmark_rust() {
  title "Running rust benchmarks"
  cd "${RUST_DIR}"

  rust_with_hash "sip_hash"
  rust_with_hash "fnv_hash"
  rust_with_hash "murmur_hash"

  printf "\n"
}

function benchmark_cpp() {
  title "Running cpp benchmarks"
  cd "${CPP_DIR}"

  printf "\n"
}

function benchmark_dlang() {
  title "Running dlang benchmarks"
  cd "${DLANG_DIR}"

  printf "\n"
}

benchmark_rust
benchmark_cpp
benchmark_dlang

title "All benchmarks done"
