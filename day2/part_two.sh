#!/bin/bash

while read LINE; do

	echo "$LINE" | cut -d':' -f2 | sed 's/[;,]/\n/g' | awk {'print $1OFS$2'} | sort -nr | while read GAME; do

		[[ "$GAME" =~ "bl" && -z "$BLUE" ]] && BLUE=1 && echo "$GAME" | cut -d' ' -f1
		[[ "$GAME" =~ "gr" && -z "$GREEN" ]] && GREEN=1 && echo "$GAME" | cut -d' ' -f1
		[[ "$GAME" =~ "red" && -z "$RED" ]] && RED=1 && echo "$GAME" | cut -d' ' -f1
		[[ -n "$GREEN" && -n "$BLUE" && -n "$RED" ]] && break

	done | paste -sd'*'

done <input.txt | paste -sd'+' | bc
