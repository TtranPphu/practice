sh initialize.sh

docker compose build --parallel

docker compose up -d devcontainer --remove-orphans && \
docker exec \
  --workdir /workspaces/practice \
  practice-devcontainer-1 \
  sh .devcontainer/post-create.sh && \
docker exec -it \
  --workdir /workspaces/practice \
  practice-devcontainer-1 \
  nvim

docker compose stop

