#!/bin/bash

# Run from project root
DEPLOY_PATH=./target/deploy
OUTPUT_PATH=./paddlers-frontend/benchmarks/app_size_stats.txt

cargo web deploy --release -p paddlers-frontend

cargo pkgid -p paddlers-frontend | cut -d# -f2 | cut -d: -f2 | tr -d '\n' >> $OUTPUT_PATH
echo -n " " >> $OUTPUT_PATH
du -s $DEPLOY_PATH | cut -f 1 | tr -d '\n' >> $OUTPUT_PATH
echo -n " " >> $OUTPUT_PATH
du -s $DEPLOY_PATH/paddlers-frontend.wasm | cut -f 1 | tr -d '\n' >> $OUTPUT_PATH
echo -n " " >> $OUTPUT_PATH
du -s $DEPLOY_PATH/paddlers-frontend.js | cut -f 1 | tr -d '\n' >> $OUTPUT_PATH
echo "" >> $OUTPUT_PATH