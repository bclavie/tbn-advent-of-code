#!/bin/sh
if [ $# -eq 0 ]; then
  echo "Please provide a year as an argument"
  exit 1
fi

year=$1
mkdir -p "$year"
for i in {1..24}; do
  folder_name=$(printf "day%02d" "$i")
  mkdir -p "$year/$folder_name"
  touch "$year/$folder_name/.gitkeep"
done
