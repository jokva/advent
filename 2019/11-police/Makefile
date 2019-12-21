all: intcode.so
	LUA_CPATH=$(PWD)/intcode.so lua5.2 11-police.lua < input.txt

intcode.so: intcode.cpp
	g++ -O2 -std=c++14 -shared -fPIC -I/usr/include/lua5.2 -o intcode.so intcode.cpp -lluabind
