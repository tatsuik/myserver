#!/bin/bash -x
set -e
export USER=dev
export HOME=/home/$USER
cd /app

uid=$(stat -c "%u" .)
gid=$(stat -c "%g" .)

if [ "$uid" -ne 0 ]; then
    if [ "$(id -g $USER)" -ne $gid ]; then
        getent group $gid >/dev/null 2>&1 || groupmod -g $gid $USER
        chgrp -R $gid $HOME
    fi
    if [ "$(id -u $USER)" -ne $uid ]; then
        usermod -u $uid $USER
    fi
fi

if [ "$ENV" = 'UNIT' ]; then
    exec setpriv --reuid=$uid --regid=$gid --init-groups cargo test
else
    /bin/bash
fi
