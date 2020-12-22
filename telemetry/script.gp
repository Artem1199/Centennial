
plot "< tail -n 500 data.dat" u 2 title "Ry Estimate Value" with lines, \
 "" u 5 title "Raw accel Y" with lines, \

while (1) {
	replot
	pause .1
}
