#!/bin/bash

echo "Splitting each CSV file into two parts..."
cd data
for file in *.csv; do
    lines=$(wc -l < "$file")
    half=$((lines / 2))

    header=$(head -n 1 "$file")

    head -n $half "$file" > "${file%.csv}_part1.csv"

    echo "$header" > "${file%.csv}_part2.csv"
    tail -n +$((half + 1)) "$file" >> "${file%.csv}_part2.csv"

    rm "$file"
done
cd ..
echo "Done!"