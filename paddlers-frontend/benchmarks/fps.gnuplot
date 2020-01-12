set terminal pdf
set output "fps.pdf"

set xlabel "Version"
set ylabel "FPS"
set yrange [0:60]

# set xlabel "Date"
# set timefmt "%s"
# set format x "%m/%d/%Y %H:%M:%S"
# set xdata time

set key outside;
set key center top;

set key maxcolumns 2 maxrows 5

plot for [IDX=0:7] 'frame_delays.manual.data' i IDX u 7:xticlabels(2) w linespoints lw 2 title columnheader(1)