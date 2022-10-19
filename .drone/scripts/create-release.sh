#!/usr/bin/env bash
set -ex

.drone/scripts/setup-pbmpr.sh
sudo apt-get install cargo gh jq -y

version="$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')"
echo "${github_api_key}" | gh auth login --with-token
gh release create "v${version}" --title "v${version}" --target "${DRONE_COMMIT_SHA}"

cargo publish

# vim: set sw=4 expandtab:
