#! /usr/bin/env bash

SCRIPTS_DIR="$(cd "$(dirname "$0")" || exit; pwd -P)"
PROJECT_ROOT_DIR="$(dirname "${SCRIPTS_DIR}")"
grep -E '^version =' "${PROJECT_ROOT_DIR}/Cargo.toml" | cut -d'"' -f 2
