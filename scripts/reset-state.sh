#!/usr/bin/env bash
set -eou pipefail

platform="$(uname)"
declare -a dirs;
if [ "$platform" == "Darwin" ]; then
    dirs=(
        "$HOME/.radicle"
        "$HOME/Library/Application Support/Radicle Upstream"
        "$HOME/Library/Application Support/xyz.radicle.radicle"
        "$HOME/Library/Application Support/xyz.radicle.radicle-upstream"
        "$HOME/Library/Preferences/radicle-upstream.monadic.xyz.plist"
        "$HOME/Library/Saved Application State/radicle-upstream.monadic.xyz.savedState"
    );
elif [ "$platform" == "Linux" ]; then
    config_home="${XDG_CONFIG_HOME:-$HOME/.config}"
    data_home="${XDG_DATA_HOME:-$HOME/.local/share}"

    dirs=(
        "$HOME/.radicle"
        "$config_home/radicle"
        "$config_home/Radicle Upstream"
        "$data_home/radicle"
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
