--- 
layout: note
title: Command Line to Remember 
---

## GitHub

1. `git clean -f` - Removes Unstaged UnTracked files
1. `git checkout .` - Removes Unstaged Tracked files
1. `git reset --hard` - Removes Staged Tracked and UnStaged Tracked
1. `git stash -u` - Removes all changes

- `git stash pop` - Reapply the changes that were removed with `git stash`
- `git commit --amend` - Adds the staged file into the last commit

## UV

Checkout UV [here](https://docs.astral.sh/uv/)

- `uv add <package-name>` — Install `<package-name`
- `source .venv/bin/activate` — UV creates a virtual environment 
- `deactivate` — To deactivate the environment
- `uv venv --python 3.11` — select the python version (3.13 in this case)
- `uv pip install -r <requirements>` — `<requirements>` can be:
    - `requirements.txt`
    - `pyproject.toml`
- `uv pip freeze` — list the dependencies in `requirements.txt` format
- `uv pip sync pyproject.toml` — sync an environment with a pyproject.toml file
- `uv run --with jupyter lab` — start a Jupyter server
