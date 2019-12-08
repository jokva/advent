using System.Linq;

class Program {

    const int position = 0;
    const int immediate = 1;

    enum Op : int {
        halt    = 99,
        add     = 1,
        mul     = 2,
        get     = 3,
        put     = 4,
        jump    = 5,
        jmpnot  = 6,
        less    = 7,
        eq      = 8,
    }

    static int[] parse_opcode(string s) {
        var op = Enumerable.Repeat(position, 3).ToArray();
        op[0] = int.Parse(s.Substring(3));
        op[1] = s[2] - '0';
        op[2] = s[1] - '0';
        return op;
    }

    static int read(int[] memory, int val, int mode) {
        if (mode == position)
            return memory[val];
        return val;
    }

    static int[] eval(int[] memory, int pc, int input) {
        while (true) {
            var op = parse_opcode(memory[pc].ToString("D5"));
            switch ((Op)op[0]) {
                case Op.halt:
                    return memory;

                case Op.add: {
                    var op1 = memory[pc + 1];
                    var op2 = memory[pc + 2];
                    var dst = memory[pc + 3];
                    memory[dst] = read(memory, op1, op[1])
                                + read(memory, op2, op[2])
                                ;
                    pc += 4;
                    break;
                }

                case Op.mul: {
                    var op1 = memory[pc + 1];
                    var op2 = memory[pc + 2];
                    var dst = memory[pc + 3];
                    memory[dst] = read(memory, op1, op[1])
                                * read(memory, op2, op[2])
                                ;
                    pc += 4;
                    break;
                }

                case Op.get: {
                    var dst = memory[pc + 1];
                    memory[dst] = input;
                    pc += 2;
                    break;
                }

                case Op.put: {
                    var src = memory[pc + 1];
                    var val = read(memory, src, op[1]);
                    System.Console.WriteLine($"{val}");
                    pc += 2;
                    break;
                }

                case Op.jump: {
                    var op1 = memory[pc + 1];
                    var op2 = memory[pc + 2];
                    if (read(memory, op1, op[1]) != 0)
                        pc = read(memory, op2, op[2]);
                    else
                        pc += 3;

                    break;
                }

                case Op.jmpnot: {
                    var op1 = memory[pc + 1];
                    var op2 = memory[pc + 2];
                    if (read(memory, op1, op[1]) == 0)
                        pc = read(memory, op2, op[2]);
                    else
                        pc += 3;

                    break;
                }

                case Op.less: {
                    var op1 = memory[pc + 1];
                    var op2 = memory[pc + 2];
                    var dst = memory[pc + 3];
                    var lhs = read(memory, op1, op[1]);
                    var rhs = read(memory, op2, op[2]);
                    memory[dst] = lhs < rhs ? 1 : 0;
                    pc += 4;
                    break;
                }

                case Op.eq: {
                    var op1 = memory[pc + 1];
                    var op2 = memory[pc + 2];
                    var dst = memory[pc + 3];
                    var lhs = read(memory, op1, op[1]);
                    var rhs = read(memory, op2, op[2]);
                    memory[dst] = lhs == rhs ? 1 : 0;
                    pc += 4;
                    break;
                }
            }
        }
    }

    static int[] readprogram() {
        return System.Console.ReadLine()
            .Split(',')
            .Select(x => int.Parse(x))
            .ToArray()
        ;
    }

    static void Main(string[] args) {
        var memory = readprogram();
        eval((int[])memory.Clone(), 0, 1);
        eval((int[])memory.Clone(), 0, 5);
    }
}
