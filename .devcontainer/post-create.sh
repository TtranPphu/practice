#! /usr/bin/sh

pip install --no-warn-script-location -r python/requirement
uv python install 3.11 3.12 3.13
