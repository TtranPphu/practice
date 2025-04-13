#! /usr/bin/bash

cp .devcontainer/docker-compose.yml.sample .devcontainer/docker-compose.yml
sed -i "s/<host-user-name>/$USER/g" .devcontainer/docker-compose.yml

# dummy nvim config
mkdir ~/.config/nvim 2>/dev/null | true
