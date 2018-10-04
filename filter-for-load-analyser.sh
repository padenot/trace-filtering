#!/bin/sh

# filter a trace file to only keep budget and overall callback duration, to feed
# to https://padenot.github.io/msg-load-analyzer/

if [ -z $1 ]
then
  echo "Usage:"
  echo "\t./$0 trace-file.json"
  exit 1
fi

echo "["
egrep "(budget|DataCallback)" $1
