#!/bin/sh

set -eu

if [ -z "$1" ]; then
  echo 'Please specify image version.'
  exit 1
fi

docker build . -t docker.genya0407.net/reing:$1
docker push docker.genya0407.net/reing:$1
