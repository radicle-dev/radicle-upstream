#!/usr/bin/env bash

set -euo pipefail

git submodule update --init --remote
git submodule foreach "git fetch --all"
git submodule foreach "git checkout -B dev -t origin/dev"
git submodule foreach "git checkout master"
git submodule foreach "git pull origin master"
