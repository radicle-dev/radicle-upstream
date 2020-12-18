#!/usr/bin/env bash
set -eou pipefail

if [ ! -z "${RAD_HOME:-}" ]; then
    home=$RAD_HOME;
else
    home=$HOME;
fi

if pgrep radicle-proxy; then
    echo "You need to stop the proxy before switching the identity!"
    exit 1
fi

echo "Select the identity you want to switch to or opt to create a new one:"

shopt -s extglob
select id in $home/.radicle/identities/!(current)/ "NEW IDENTITY"
do
case "$id" in
    "NEW IDENTITY" )
        unlink $home/.radicle/identities/current
        echo "Removed the symlink"
        break
        ;;
    * )
        ln -sfn $id $home/.radicle/identities/current
        echo "Switched to identity $id"
        break
        ;;
esac
done