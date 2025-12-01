#!/bin/sh

# Require: year and number of days
if [ $# -lt 2 ]; then
  echo "Usage: $0 <year> <days>"
  exit 1
fi

year=$1
days=$2

# Ensure 'days' is a number
case $days in
  ''|*[!0-9]*)
    echo "Days must be a numeric value"
    exit 1
    ;;
esac

mkdir -p "$year"

i=1
while [ $i -le $days ]; do
  folder_name=$(printf "day%02d" "$i")
  mkdir -p "$year/$folder_name"
  touch "$year/$folder_name/.gitkeep"
  i=$((i + 1))
done

