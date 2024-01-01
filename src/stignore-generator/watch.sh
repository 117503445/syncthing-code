#!/usr/bin/env bash

./generate.sh

inotifywait --format "%f" -e 'modify,create,delete' -m -r /workspace |\
while read line; do
  if [[ "$line" == ".gitignore" ]]; then
    # echo ".gitignore update"
    # 1. Run the generate script
    ./generate.sh
    # 2. Check if there is any content difference against current stignore. If
    #    there is then replace it.
  fi
done