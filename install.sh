#!/bin/sh
set -e
mkdir -p "$HOME/.local/bin"
curl -fsSL https://github.com/trtlang/trt/releases/latest/download/trt -o "$HOME/.local/bin/trt"
chmod +x "$HOME/.local/bin/trt"
echo "Installed TRT."
