#!/usr/bin/env bash
set -e


# cd elixir-ls
# mix deps.get
# mix elixir_ls.release -o ../elixir-ls-release

wget https://github.com/elixir-lsp/elixir-ls/releases/download/v0.11.0/elixir-ls-1.13-22.3.zip 
unzip elixir-ls-*.zip -d elixir-ls-release
