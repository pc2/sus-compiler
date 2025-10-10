#!/bin/bash
# Usage: ./clear_fixed_panics.sh /path/to/crash_dumps
# Must contain a subdirectory crash_dumps

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <crash dumps directory>"
    exit 1
fi

CRASH_DUMPS_FOLDER="$1"
cargo build
SUS_COMPILER="/home/lennart/Desktop/sus-compiler/target/debug/sus_compiler"

if [ ! -d "$CRASH_DUMPS_FOLDER" ]; then
    echo "Directory $CRASH_DUMPS_FOLDER does not exist."
    exit 1
fi

echo "Directory is $CRASH_DUMPS_FOLDER/*"
for dump in "$CRASH_DUMPS_FOLDER"/*
do
    echo "Checking $dump..."

    
    cd "$dump"
    "$SUS_COMPILER" --codegen --no-redump > /dev/null 2>&1
    status=$?
    cd ..

    echo "Return code: $status"
    if [ $status -eq 0 ]; then
        echo "No crash in $dump, deleting..."
        rm -r "$dump"
    else
        echo "Crash or error (code $status) in $dump, keeping."
    fi
done

echo "All dumps handled"
