#!/bin/bash

sed 's/[^0-9]//g' input.txt | while read LINE; do
	echo "${LINE::1}${LINE:(-1)}"
done | paste -sd'+' | bc
