#!/bin/bash
# A start to automatically export duck SVGs to PNGs. Not complete, yet.
# Ideally, in the future, PNGs need not even be checked in with git!
STATIC=../../static
DIALOGUE_SIZE=500
inkscape --without-gui --file "roger/roger.svg" --export-png "${STATIC}/ducks/roger_large.png" --export-width $DIALOGUE_SIZE
inkscape --without-gui --file "roger/roger_sad.svg" --export-png "${STATIC}/ducks/roger_sad.png" --export-width $DIALOGUE_SIZE
inkscape --without-gui --file "roger/roger_astonished.svg" --export-png "${STATIC}/ducks/roger_astonished.png" --export-width $DIALOGUE_SIZE
inkscape --without-gui --file "roger/roger_obedient.svg" --export-png "${STATIC}/ducks/roger_obedient.png" --export-width $DIALOGUE_SIZE
inkscape --without-gui --file "roger/roger_celebrating.svg" --export-png "${STATIC}/ducks/roger_celebrating.png" --export-width $DIALOGUE_SIZE

