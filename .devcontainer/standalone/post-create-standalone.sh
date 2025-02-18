#! /usr/bin/sh

sh .devcontainer/post-create-base.sh
cp .devcontainer/standalone/.p10k.zsh ~/.p10k.zsh
cp .devcontainer/.zshrc ~/.zshrc