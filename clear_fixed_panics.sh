#!/usr/bin/env bash
# set -Eeuo pipefail
IFS=$'\n\t'

# Usage: ./clear_fixed_panics.sh /absolute/or/relative/path/to/crash_dumps

########################################
# Argument validation
########################################

if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <crash_dumps directory>"
    exit 1
fi

# Resolve to absolute canonical path
CRASH_DUMPS_FOLDER="$(realpath -e "$1")" || {
    echo "Error: Cannot resolve path '$1'"
    exit 1
}

if [[ ! -d "$CRASH_DUMPS_FOLDER" ]]; then
    echo "Error: '$CRASH_DUMPS_FOLDER' is not a directory"
    exit 1
fi

if [[ "$(basename "$CRASH_DUMPS_FOLDER")" != "crash_dumps" ]]; then
    echo "Error: Provided directory must be named exactly 'crash_dumps'"
    exit 1
fi

########################################
# Build compiler safely
########################################

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR"

cd "$PROJECT_ROOT"

echo "Building compiler..."
cargo build

SUS_COMPILER="$PROJECT_ROOT/target/debug/sus_compiler"

if [[ ! -x "$SUS_COMPILER" ]]; then
    echo "Error: Compiler not found or not executable at:"
    echo "  $SUS_COMPILER"
    exit 1
fi

########################################
# Process crash dumps safely
########################################

echo "Processing crash dumps in:"
echo "  $CRASH_DUMPS_FOLDER"
echo

shopt -s nullglob

found_any=false

for dump in "$CRASH_DUMPS_FOLDER"/*; do
    [[ -d "$dump" ]] || continue
    found_any=true

    echo "Checking: $dump"

    # Run compiler in isolated subshell
    (
        cd "$dump" || exit 127
        "$SUS_COMPILER" *.sus --no-redump -o /dev/null 2> /dev/null
    )
    status=$?

    echo "Return code: $status"

    if [[ $status -eq 0 ]] || [ $status -eq 1 ]; then
        echo "No crash detected. Deleting:"
        echo "  $dump"

        # Double-safety: ensure we're deleting inside crash_dumps
        case "$dump" in
            "$CRASH_DUMPS_FOLDER"/*)
                rm -rf -- "$dump"
                ;;
            *)
                echo "Refusing to delete unexpected path!"
                exit 1
                ;;
        esac
    else
        echo "Crash or error (code $status). Keeping."
    fi

    echo
done

if [[ "$found_any" = false ]]; then
    echo "No crash dump directories found."
fi

echo "All dumps handled."