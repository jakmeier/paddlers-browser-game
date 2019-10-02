#!/bin/bash
LGREEN="#99e540"
GREEN="#0fa100"
DGREEN="#0c7f00"
LBLUE="#4099e5"
BLUE="#000fa1"
DBLUE="#000c7f"
colors=(
    $LGREEN
    $GREEN
    $DGREEN
    $LBLUE
    $BLUE
    $DBLUE
)
filenames=(
    "green_frame_1"
    "green_frame_2"
    "green_frame_3"
    "blue_frame_1"
    "blue_frame_2"
    "blue_frame_3"
)
for i in `seq 0 5`;
do
    cat "frame.svg" | sed "s/#b3b3b3/${colors[i]}/g" | inkscape --without-gui --file /dev/stdin  --export-png "${filenames[i]}.png" --export-width 150
    mv ${filenames[i]}.png ../../../static/gui/abilities/${filenames[i]}.png
done