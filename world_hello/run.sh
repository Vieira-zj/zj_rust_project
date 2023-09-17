#!/bin/bash
set -eu

function rust_setup {
    local ver="1_72"
    rm /Users/jinzheng/.cargo
    curl https://sh.rustup.rs -sSf | sh
    ln -s /Users/jinzheng/Workspaces/.cargo${version} /Users/jinzheng/.cargo
}

function rust_build {
    cargo build
}

echo "hello rust"
