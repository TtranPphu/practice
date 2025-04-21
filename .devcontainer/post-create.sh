#! /usr/bin/sh

cp .devcontainer/pre-commit .git/hooks/pre-commit

(cd python; . $HOME/.local/bin/env && uv sync)
