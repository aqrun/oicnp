#!/bin/bash

user=$OICNP_USER
host=$OICNP_HOST

if [ ! -n "$user" ]; then
echo "[ERR:]OICNP_USER not set!"
exit
fi

if [ ! -n "$host" ];  then
echo "[ERR:]OICNP_HOST not set!"
exit
fi

echo "OICNP_HOST: $host"

echo "current at: $(pwd)"

source_file=$(pwd)/target/release/oicnp_links
dist_file=$user@$host:/workspace/apps/oicnp/

echo "copy fill from $source_file to $dist_file"
scp $source_file $dist_file