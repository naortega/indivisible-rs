#!/bin/bash

echo -n "Prime finding..."
if [[ $(./target/release/indivisible 5) == 11 ]]
then
	echo " pass"
else
	echo " FAIL"
	exit 1
fi

echo -n "Positive prime test..."
if ./target/release/indivisible -t 11
then
	echo " pass"
else
	echo " FAIL"
	exit 1
fi

echo -n "Negative prime test..."
if ! ./target/release/indivisible -t 9
then
	echo " pass"
else
	echo " FAIL"
	exit 1
fi
