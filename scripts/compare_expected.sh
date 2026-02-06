#!/bin/bash

mkdir -p output/expected

for file in output/*.json output/benchmark/*.json; do
    [ -f "$file" ] || continue

    basename=$(basename "$file")

    if [[ "$basename" =~ ^(.+)_all_[0-9]+_cpus\.json$ ]]; then
        expected="output/expected/expected_${BASH_REMATCH[1]}.json"
    elif [[ "$basename" =~ ^(.+)_[0-9]+_cpus\.json$ ]]; then
        expected="output/expected/expected_${BASH_REMATCH[1]}.json"
    else
        continue
    fi

    if [ -f "$expected" ]; then
        if python3 -c "
import json, sys
with open('$file') as f1, open('$expected') as f2:
    data1 = sorted(json.load(f1), key=str)
    data2 = sorted(json.load(f2), key=str)
    sys.exit(0 if data1 == data2 else 1)
" 2>/dev/null; then
            echo "✅ $basename matches expected"
        else
            echo "❌ $basename differs from expected"
        fi
    else
        echo "⚠️  $basename: no expected file found"
    fi
done