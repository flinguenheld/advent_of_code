#!/bin/bash

# Impossible with all values

while read LINE; do

	GOODS=$(echo "$LINE" | awk -F'|' '{print $2}')
	echo "$LINE" | awk -F'[:|]' '{print $2}' | sed 's/\s/\n/g;' | sed -e '/^$/d' | while read PLAYED; do

		[[ " $GOODS " =~ " $PLAYED " ]] && echo "$PLAYED" # Spaces for 1-9

	done | wc -l

done <input.txt | sed 's/$/:1/' >tmp.pouet

for i in $(seq $(wc --lines tmp.pouet | cut -d' ' -f1)); do

	LINE=$(cat tmp.pouet | sed -n "${i}p")

	WIN=$(echo "$LINE" | cut -d':' -f1)
	NB_CARDS=$(echo "$LINE" | cut -d':' -f2 | bc)

	TOTAL=$((WIN * NB_CARDS))

	for j in $(seq $NB_CARDS); do
		for k in $(seq $WIN); do

			SUB_LINE=$(cat tmp.pouet | sed -n "$((i + k))p")

			SUB_WIN=$(echo "$SUB_LINE" | cut -d':' -f1)
			SUB_NB_CARDS=$(echo "$SUB_LINE" | cut -d':' -f2 | bc)
			sed -i "$((i + k)) s/.*/$SUB_WIN:$((SUB_NB_CARDS + 1))/" tmp.pouet
		done
	done

	sed -i "$i s/.*/$WIN:$NB_CARDS/" tmp.pouet
	echo "Current line: $i done"
done

cat tmp.pouet | awk -F':' '{print $2}' | paste -sd'+' | bc
