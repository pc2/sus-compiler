#!/bin/bash
# Usage: ./clear_fixed_panics.sh /path/to/sus_home
# Must contain a subdirectory crash_dumps

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <sus_home_directory>"
    exit 1
fi

SUS_HOME="$1"
cargo build
SUS_COMPILER="/home/lennart/Desktop/sus-compiler/target/debug/sus_compiler"

if [ ! -d "$SUS_HOME/crash_dumps" ]; then
    echo "Directory $SUS_HOME/crash_dumps does not exist."
    exit 1
fi

echo "Directory is $SUS_HOME/crash_dumps/*"
for dump in "$SUS_HOME"/crash_dumps/*
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
