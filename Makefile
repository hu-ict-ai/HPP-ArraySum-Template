all:
	g++ -O2 -fopenmp -lomp -o arraySum arraySum.cc

run: arraySum
	./arraySum
