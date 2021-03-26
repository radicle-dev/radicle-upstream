#!/usr/bin/env bash
set -eou pipefail

platform="$(uname)"
declare -a dirs;
if [ "$platform" == "Darwin" ]; then
    dirs=(
        "$HOME/.radicle"
        "$HOME/Library/Application Support/Radicle Upstream"
        "$HOME/Library/Application Support/xyz.radicle.radicle-link"
        "$HOME/Library/Application Support/xyz.radicle.radicle-upstream"
        "$HOME/Library/Preferences/xyz.radicle.radicle-upstream.plist"
        "$HOME/Library/Saved Application State/xyz.radicle.radicle-upstream.savedState"
    );
elif [ "$platform" == "Linux" ]; then
    config_home="${XDG_CONFIG_HOME:-$HOME/.config}"
    data_home="${XDG_DATA_HOME:-$HOME/.local/share}"

    dirs=(
        "$HOME/.radicle"
        "$config_home/radicle-link"
        "$config_home/Radicle Upstream"
        "$data_home/radicle-link"
        "$data_home/radicle-upstream"
    );
else
    echo "Unsupported platform $platform"
    exit 1
fi

echo "This will irrevocably destroy the following directories:"
echo
printf '%s\n' "${dirs[@]}"
echo

read -r -p "Are you sure you want to continue? [yes/no]: " confirm
case "$confirm" in
    [yY][eE][sS] )
        rm -rfv "${dirs[@]}"
        echo "Done"
        ;;
    * )
        echo "Ok, I won't touch your data"
        ;;
esac
