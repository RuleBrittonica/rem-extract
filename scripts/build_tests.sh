#!/bin/bash

# First we need to remove the old test files if they exist
rm -rf ./input
rm -rf ./output
rm -rf ./expected_output

# Then make the starting dirs
mkdir -p ./input
mkdir -p ./output
mkdir -p ./expected_output

# Now we need to generate the input files, using the python scripts
python3 ./extract_tests.py
python3 ./extract_tests_2.py
python3 ./extract_tests_3.py
python3 ./fixup_semicolons.py
