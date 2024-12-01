#!/bin/bash

sed 's/one/o1e/g; s/two/t2o/g; s/three/t3e/g; s/four/4/g; s/five/5e/g; s/six/6/g; s/seven/7n/g; s/eight/e8t/g; s/nine/n9e/g' input.txt | sed 's/[^0-9]//g' | while read LINE; do
	echo "${LINE::1}${LINE:(-1)}"
done | paste -sd'+' | bc
