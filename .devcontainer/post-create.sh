#! /usr/bin/sh

echo $HOST_USERNAME
sed -i "s/$HOST_USERNAME/<host-username>/g" docker-compose.yml

cp .devcontainer/pre-commit .git/hooks/pre-commit
git config core.editor "nvim"

(cd python; . $HOME/.local/bin/env && uv sync)
