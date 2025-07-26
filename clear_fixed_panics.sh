#!/bin/bash
# Usage: ./clear_fixed_panics.sh /path/to/crash_dumps

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <crash_dumps_directory>"
    exit 1
fi

CRASHDUMPS_DIR="$1"
SUS_COMPILER="/home/lennart/Desktop/sus-compiler/target/release/sus_compiler"

if [ ! -d "$CRASHDUMPS_DIR" ]; then
    echo "Directory $CRASHDUMPS_DIR does not exist."
    exit 1
fi

for dump in "$CRASHDUMPS_DIR"/*; do
    [ -d "$dump" ] || continue
    echo "Checking $dump..."

    
    cd "$dump"
    "$SUS_COMPILER" --codegen --no-redump > /dev/null 2>&1
    status=$?
    cd ..

    echo "Return code: $status"
    if [ $status -eq 0 ]; then
        echo "No crash in $dump, deleting..."
        rm -rf "$dump"
    else
        echo "Crash or error (code $status) in $dump, keeping."
    fi
done
