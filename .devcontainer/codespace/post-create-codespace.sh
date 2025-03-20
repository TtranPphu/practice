#! /usr/bin/sh

cd python; uv python install "$(< .python-version)" ; uv sync; cd ..
cp .devcontainer/codespace/.p10k.zsh ~/.p10k.zsh
