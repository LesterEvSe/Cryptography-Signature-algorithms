#!/bin/bash

CIRCUIT_NAME=$1
JSON_NAME=$2

bash scripts/compile-circuit.sh $CIRCUIT_NAME
bash scripts/gen-witness.sh $CIRCUIT_NAME $JSON_NAME
