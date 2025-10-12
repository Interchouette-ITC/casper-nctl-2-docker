#!/bin/bash
set -m

# Activate virtual environment and install required packages
python3 -m venv $VIRTUAL_ENV
# pip install --upgrade pip
pip install supervisor toml tomlkit
