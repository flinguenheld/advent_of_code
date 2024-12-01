#!/bin/bash

# Impossible with all values

ARRAY=($(while read LINE; do

	GOODS=$(echo "$LINE" | awk -F'|' '{print $2}')
	echo "$LINE" | awk -F'[:|]' '{print $2}' | sed 's/\s/\n/g;' | sed -e '/^$/d' | while read PLAYED; do

		[[ " $GOODS " =~ " $PLAYED " ]] && echo "$PLAYED" # Spaces for 1-9

	done | wc -l

done <input.txt | sed 's/$/:1/'))

# --
for i in $(seq "${#ARRAY[@]}"); do

	((i--))
	LINE=${ARRAY[i]}

	WIN=$(echo "$LINE" | cut -d':' -f1)
	NB_CARDS=$(echo "$LINE" | cut -d':' -f2)

	TOTAL=$((WIN * NB_CARDS))

	for j in $(seq $NB_CARDS); do
		for k in $(seq $WIN); do

			SUB_LINE="${ARRAY[$((i + k))]}"

			SUB_WIN=$(echo "$SUB_LINE" | cut -d':' -f1)
			SUB_NB_CARDS=$(echo "$SUB_LINE" | cut -d':' -f2)

			ARRAY[$((i + k))]="$SUB_WIN:$((SUB_NB_CARDS + 1))"
		done
	done

	echo "Current line: $i done"
done

echo "${ARRAY[@]}" | sed 's/\s/\n/g;' | awk -F':' '{print $2}' | paste -sd'+' | bc
