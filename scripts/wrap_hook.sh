#!/usr/bin/env bash

name="$1"
shift
dir="./log_scripts"
filename="prek_statuses"
file="$dir/$filename"

touch "$file"
"$@"
status=$?
if [ "$status" -eq 0 ]; then
    result="PASSED"
else 
    result="FAILED (exit $status)"
fi

printf '%s|%s\n' "$name" "$result" >> $file

exit "$status"