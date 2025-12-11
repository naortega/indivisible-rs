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

## TEST
((tests++))
echo -n "${tests}: Find all prime numbers before 70..."
expect="2 3 5 7 11 13 17 19 23 29 31 37 41 43 47 53 59 61 67 "

if [ "$("$BINARY" -v 70 | tr '\n' ' ')" = "$expect" ]
then
	echo " pass"
	((passed++))
else
	echo " FAIL"
fi

## TEST
((tests++))
echo -n "${tests}: Find last prime before 1,000,000,000..."
expect="999999937"

if [ "$("$BINARY" 1000000000)" = "$expect" ]
then
	echo " pass"
	((passed++))
else
	echo " FAIL"
fi

## TEST
((tests++))
echo -n "${tests}: 11 is prime..."
if "$BINARY" -t 11
then
	echo " pass"
	((passed++))
else
	echo " FAIL"
fi

## TEST
((tests++))
echo -n "${tests}: 9 is not prime..."
if ! "$BINARY" -t 9
then
	echo " pass"
	((passed++))
else
	echo " FAIL"
fi

## RESULTS
echo "Results: $passed/$tests"

if [ $passed -ne $tests ]
then
	exit 1
fi
