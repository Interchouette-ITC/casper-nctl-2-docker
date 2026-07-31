#!/bin/bash
set -euo pipefail

python3 -m venv "${VIRTUAL_ENV}"
pip install supervisor toml tomlkit
