#! /usr/bin/sh

cp .devcontainer/pre-commit .git/hooks/pre-commit

(cd python; uv sync)
