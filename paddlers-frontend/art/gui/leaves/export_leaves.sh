#!/bin/bash

inkscape --without-gui --file "5_leaves_v.svg" --export-png "100px_top.png" --export-width 200 --export-area=0:1413:1347:1806
inkscape --without-gui --file "5_leaves_v.svg" --export-png "100px_mid.png" --export-width 200 --export-area=0:667:1347:1413
inkscape --without-gui --file "5_leaves_v.svg" --export-png "100px_bot.png" --export-width 200 --export-area=0:0:1347:667

inkscape --without-gui --file "3_leaves_v.svg" --export-png "50px_top.png" --export-width 100 --export-area=0:1500:1000:2000
inkscape --without-gui --file "3_leaves_v.svg" --export-png "50px_mid.png" --export-width 100 --export-area=0:700:1000:1500
inkscape --without-gui --file "3_leaves_v.svg" --export-png "50px_bot.png" --export-width 100 --export-area=0:0:1000:700