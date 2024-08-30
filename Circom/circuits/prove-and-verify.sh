#!/bin/bash

CIRCUIT_NAME=$1
JSON=$2
SETUP_POWERS=$3

bash scripts/export-keys.sh $CIRCUIT_NAME $SETUP_POWERS
bash scripts/prove.sh $CIRCUIT_NAME $JSON
bash scripts/verify.sh $CIRCUIT_NAME