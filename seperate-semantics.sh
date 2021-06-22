#!/bin/sh

for file in $1/*.json
do
  filename=$(basename "$file")
  cat "$file" | jq -c '.timelineObjects[]' > $2/$filename
done
