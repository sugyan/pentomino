#!/bin/bash

set -eux

cd "$(dirname "$0")"
wasm-pack build --target web
