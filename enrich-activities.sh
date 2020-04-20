#!/bin/sh

for file in $1/*.json
do
  filename=$(basename "$file")
  echo "Processing $file..."
  cat "$file" | jq -c '.timelineObjects[]' | while read line
  do
    es_id=`echo $line | jq '[.placeVisit.duration.startTimestampMs, .activitySegment.duration.startTimestampMs] | join("")'`
    es_type=`echo $line | jq 'keys[]' | tr '[:upper:]' '[:lower:]'`

    echo "{ \"index\" : { \"_index\" : $es_type, \"_id\" : $es_id } }" >> $2/$filename
    echo "$line" >> $2/$filename
  done
done
