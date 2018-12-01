all: 1 2

1: freq-1
	./freq-1 < input.txt

2: freq-2
	./freq-2 < input.txt

freq-1: freq-1.rs
	rustc freq-1.rs

freq-2: freq-2.rs
	rustc freq-2.rs

clean:
	rm -f freq-1 freq-2
