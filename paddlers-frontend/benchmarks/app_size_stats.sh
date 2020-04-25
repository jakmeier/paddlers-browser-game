#!/bin/bash

# Run from project root
DEPLOY_PATH=./target/deploy
OUTPUT_PATH=./paddlers-frontend/benchmarks/app_size_stats.txt

cd paddlers-frontend;
cargo web deploy --release -p paddlers-frontend
cd ..;

cargo pkgid -p paddlers-frontend | cut -d# -f2 | cut -d: -f2 | tr -d '\n' >> $OUTPUT_PATH
echo -n " " >> $OUTPUT_PATH
du -sb $DEPLOY_PATH | cut -f 1 | tr -d '\n' >> $OUTPUT_PATH
echo -n " " >> $OUTPUT_PATH
stat --printf="%s" $DEPLOY_PATH/paddlers-frontend.wasm >> $OUTPUT_PATH
echo -n " " >> $OUTPUT_PATH
stat --printf="%s" $DEPLOY_PATH/paddlers-frontend.js >> $OUTPUT_PATH
echo "" >> $OUTPUT_PATH