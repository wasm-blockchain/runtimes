#!/bin/bash

# set -x
set -euo pipefail

IARG=corpus/fuzz_wasmer
OARG=artifacts
APP=../target/debug/fuzz_wasmer

if [[ $# -ne 0 ]]; then
  echo "$0: expect no arguments" >&2
  exit 1
fi

cat << EOF
This script will perform four afl-fuzz runs against wasmer.  Two will
use the Singlepass backend, and two will use the Cranelift backend.
Two will use afl.rs's fuzz! macro, and two will not.  The configuration
will be displayed in the banner at the top of the screen.  Pressing
ctrl-C will start the next run.

Notice each run's stability.

Press enter to begin.
EOF

read

for BACKEND in singlepass cranelift; do
  for MACRO in '' no_macro; do
    cargo afl build --features="$BACKEND $MACRO"
    TARG="backend: $BACKEND, macro: $(if [[ -z "$MACRO" ]]; then echo yes; else echo no; fi)"
    trap '' SIGINT
    cargo afl fuzz -i "$IARG" -o "$OARG" -T "$TARG" -- "$APP"
    trap - SIGINT
  done
done
