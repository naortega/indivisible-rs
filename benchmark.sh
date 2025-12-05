#!/bin/bash

BIN="./target/release/indivisible"

if ! [ -f "$BIN" ]
then
	>&2 echo "Release build not available. Please run 'cargo build -r'."
	exit 1
fi

time "$BIN" 100000000
