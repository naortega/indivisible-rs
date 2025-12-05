#!/bin/bash

BIN="./target/release/indivisible"
TRIALS=20

if ! [ -f "$BIN" ]
then
	>&2 echo "Release build not available. Please run 'cargo build -r'."
	exit 1
fi

echo "Calculating primes up to 100,000,000"
TOTAL="0"
for _ in $(seq "$TRIALS")
do
	TIME=$(command time -f "%e" "$BIN" 100000000 2>&1 >/dev/null)
	TOTAL=$(calc "$TOTAL + $TIME")
done

AVG=$(calc "$TOTAL / $TRIALS")

echo "Average time: ${AVG}s"
