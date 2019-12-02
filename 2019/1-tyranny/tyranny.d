int fuelfuel(int x) {
    if (x <= 0) return 0;
    return x + fuelfuel((x / 3) - 2);
}

void main() {
    import std.stdio;
    import std.algorithm;
    import std.conv;
    import std.array;

    auto modules = stdin.byLine().map!(x => x.to!int).array;

    const fuel = modules.map!(x => x / 3 - 2).sum();
    stdout.writeln(fuel);

    const load = modules.map!(x => fuelfuel(x) - x).sum();
    stdout.writeln(load);
}
