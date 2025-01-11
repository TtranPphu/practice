#! /usr/bin/sh

pip install --no-warn-script-location -r .devcontainer/tools
uv python install 3.11 3.12 3.13
