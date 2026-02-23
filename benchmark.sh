#!/bin/bash

EXE="./target/release/indivisible"
OPTIONS=()
TRIALS=20

if ! [ -f "$EXE" ]
then
	>&2 echo "Release build not available. Please run 'cargo build -r'."
	exit 1
fi

if ! command -v calc &>/dev/null
then
	>&2 echo "Missing 'calc' program. Please install it for this script."
	exit 1
fi

while getopts "t:s:" opt
do
	case "$opt" in
		s)
			OPTIONS=("${OPTIONS[@]}" -s "$OPTARG")
			SIEVE=$OPTARG
			;;
		t)
			TRIALS="$OPTARG"
			;;
		*)
			>&2 echo "Uknown option $opt"
			exit 1
			;;
	esac
done

echo "Calculating primes up to 1,000,000,000"
echo "Trials: $TRIALS"
echo "Sieve segment size: ${SIEVE:-"default"}"
TOTAL="0"
for _ in $(seq "$TRIALS")
do
	TIME=$(command time -f "%e" "$EXE" "${OPTIONS[@]}" 1000000000 2>&1 >/dev/null)
	TOTAL=$(calc "$TOTAL + $TIME")
done

AVG=$(calc "$TOTAL / $TRIALS")

echo "Average time: ${AVG}s"
