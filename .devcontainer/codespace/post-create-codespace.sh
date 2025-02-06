#! /usr/bin/sh

sh .devcontainer/post-create-base.sh
cp .devcontainer/codespace/.p10k.zsh ~/.p10k.zsh
cp .devcontainer/.zshrc ~/.zshrc