#!/bin/bash

CIRCUIT_NAME=$1

bash scripts/compile-circuit.sh $CIRCUIT_NAME
bash scripts/gen-witness.sh $CIRCUIT_NAME input.json
