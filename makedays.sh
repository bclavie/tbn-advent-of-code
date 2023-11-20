#!/bin/sh
for i in {1..24}; do
  folder_name=$(printf "day%02d" "$i")
  mkdir "$folder_name"
  touch "$folder_name/.gitkeep"
done
