#! /usr/bin/sh

cd python; uv sync; cd ..
cp .devcontainer/standalone/.p10k.zsh ~/.p10k.zsh
cp .devcontainer/.zshrc ~/.zshrc
