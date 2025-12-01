#!/usr/bin/env bash

FILE="input.txt"
# FILE="input_example.txt"

NUMBER="50"
cat "$FILE" | sed 's/R//' | sed 's/L/-/' | while read -r LINE; do
  NUMBER=$(( (NUMBER + LINE) % 100 ))
  if [[ "$NUMBER" -eq "0" ]]; then
    echo "1"
  fi
done | paste -sd '+' | bc
