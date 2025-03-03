--- 
layout: page
title: Command Line to Remember 
---

## GitHub

1. `git clean -f` - Removes Unstaged UnTracked files ONLY [Type 1]
1. `git checkout .` - Removes Unstaged Tracked files ONLY [Type 2]
1. `git reset --hard` - Removes Staged Tracked and UnStaged Tracked files ONLY[Type 2, Type 3]
1. `git stash -u` - Removes all changes [Type 1, Type 2, Type 3]

- `git commit --amend [--no-edit]` - Adds the staged file into the last commit [without changing the message]
- `git fetch --prune` - updates the list of remote branches and remove any references to branches that have been deleted on the remote.

## UV

Checkout UV [here](https://docs.astral.sh/uv/)

- `uv pip install <package-name>` — Install `<package-name`
- `source .venv/bin/activate` — The virtual environment can be "activated" to make its packages available 
- `deactivate` — To deactivate the environment
- `uv venv --python 3.11` — select the python version (3.13 in this case)
- `uv pip install -r <requirements>` — `<requirements>` can be expressed in the followings:
    - `requirements.txt`
    - `pyproject.toml`
- `uv pip freeze` — list all of the packages in the environment in a requirements.txt format
- `uv pip compile pyproject.toml -o requirements.txt` — lock dependencies declared in a pyproject.toml
- `uv pip sync pyproject.toml` — sync an environment with a pyproject.toml file

## Docker

- `docker compose up -d --build` — if necessary, it recreates the images and
  run the containers 
- `docker compose stop` — stops the containers, the images are preserved
- `docker compose down --rmi all` - remove the containers and deletes the images
- `docker compose restart` — the images are unchanged, (i.e. `up` followed by `down`)
- `docker compose exec -it <service> <command>` — if command is `bash`, you enter the container
