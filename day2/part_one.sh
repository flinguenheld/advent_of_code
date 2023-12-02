#!/bin/bash

while read LINE; do

	echo "$LINE" | cut -d':' -f2 | sed 's/[;,]/\n/g' | awk {'print $1OFS$2'} | sort -nr | while read GAME; do

		NB=$(echo "$GAME" | awk '{print $1}')
		[[ "$NB" -le 12 ]] && exit 0
		[[ "$NB" -gt 14 ]] && exit 1
		[[ "$NB" -gt 13 && "$GAME" =~ "gr" ]] && exit 1
		[[ "$NB" -gt 12 && "$GAME" =~ "red" ]] && exit 1
	done

	[[ "$?" == "0" ]] && echo "$LINE" | awk -F'[ :]' '{print $2}'

done <input.txt | paste -sd'+' | bc
