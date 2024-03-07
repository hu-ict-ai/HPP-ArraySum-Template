UNAME := $(shell uname)

ifeq ($(UNAME), Darwin)
	OPT_FLAGS=-lomp
else
	OPT_FLAGS=
endif

all:
		g++ -O2 -fopenmp $(OPT_FLAGS) -o arraySum arraySum.cc

run: arraySum
	./arraySum
