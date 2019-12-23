all: 1 2


1: intcode
	./intcode input.txt tiles | awk 'NR == 1 || NR % 3 == 0' | grep '^2$$' | wc -l

2: intcode
	./play

intcode: 13-care.cpp
	g++ -std=c++14 -O2 13-care.cpp -o intcode
