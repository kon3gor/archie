#!/bin/sh

filename=$(echo $1 | awk -F '/' '{print $(NF)}' | awk -F '.' '{print $1}')
curdate=$(date +%s)
d=$(date -j -f "%F" $filename +%s)
delta=$((($curdate-$d)/(24*60*60)))
if [ "$delta" -gt "7" ]; then
	echo 1
else
	echo 0
fi
