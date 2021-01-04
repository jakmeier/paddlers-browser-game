#!/bin/bash

# Run from project root
DEPLOY_PATH=./www/dist/
OUTPUT_PATH=./paddlers-frontend/benchmarks/app_size_stats.txt

cd paddlers-frontend;
wasm-pack build
cd ../www;
npm run release
cd ..;

cargo pkgid -p paddlers-frontend | cut -d# -f2 | cut -d: -f2 | tr -d '\n' >> $OUTPUT_PATH
echo -n " " >> $OUTPUT_PATH
# du -sb $DEPLOY_PATH | cut -f 1 | tr -d '\n' >> $OUTPUT_PATH
echo -n "?" >> $OUTPUT_PATH
echo -n " " >> $OUTPUT_PATH
stat --printf="%s" $DEPLOY_PATH/*.wasm >> $OUTPUT_PATH
echo -n " " >> $OUTPUT_PATH
stat -c "%s" $DEPLOY_PATH/*.js | paste -d+ -s - | bc >> $OUTPUT_PATH
echo "" >> $OUTPUT_PATH