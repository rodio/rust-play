#!/bin/zsh
wasm-pack build --target web
python3 -m http.server
