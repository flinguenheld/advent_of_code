#!/bin/bash

while read LINE; do

	GOODS=$(echo "$LINE" | awk -F'|' '{print $2}')
	echo "$LINE" | awk -F'[:|]' '{print $2}' | sed 's/\s/\n/g;' | sed -e '/^$/d' | while read PLAYED; do

		[[ " $GOODS " =~ " $PLAYED " ]] && echo "$PLAYED" # Spaces for 1-9

	done | wc -l | while read SCORE; do

		[[ "$SCORE" -gt 2 ]] && echo $((1 << ($SCORE - 1))) || echo "$SCORE"
	done

done <input.txt | paste -sd'+' | bc
