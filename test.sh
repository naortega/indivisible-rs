#!/bin/bash

DEBUG_BIN="./target/debug/indivisible"
RELEASE_BIN="./target/release/indivisible"

if [ -f "$DEBUG_BIN" ]
then
	BINARY="$DEBUG_BIN"
elif [ -f "$RELEASE_BIN" ]
then
	BINARY="$RELEASE_BIN"
else
	>&2 echo "No valid binary found. Please compile the project."
	exit 1
fi

tests=0
passed=0

((tests++))
echo -n "${tests}: Find 5th prime number..."
if [[ $("$BINARY" 5) == 11 ]]
then
	echo " pass"
	((passed++))
else
	echo " FAIL"
fi

((tests++))
echo -n "${tests}: 11 is prime..."
if "$BINARY" -t 11
then
	echo " pass"
	((passed++))
else
	echo " FAIL"
fi

((tests++))
echo -n "${tests}: 9 is not prime..."
if ! "$BINARY" -t 9
then
	echo " pass"
	((passed++))
else
	echo " FAIL"
fi

echo "Results: $passed/$tests"
