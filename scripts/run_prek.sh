#!/usr/bin/env bash

dir="./log_scripts"
file="$dir/prek_statuses"
mkdir -p "$dir"
rm -f "$file"

./scripts/pre_prek.sh
jj squash
prek run "$@" 
status=$?

if [ -f "$file" ]; then
    awk -F '|' '
        length($1) > max {
            max = length($1)
        }
        {
            names[NR] = $1
            results[NR] = $2
        }
        END {
            for (i = 1; i <= NR; i++)
                printf "%-*s   %s\n", max, names[i], results[i]
        }
    ' "$file"
else 
    echo "no hook statuses recorded"
fi

exit $status