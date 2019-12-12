#include <algorithm>
#include <ciso646>
#include <cstdint>
#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>

class intcode {
public:
    enum class opcode {
        halt    = 99,
        add     = 1,
        mul     = 2,
        get     = 3,
        put     = 4,
        jmp     = 5,
        jmpz    = 6,
        less    = 7,
        eq      = 8,
        rela    = 9,
    };

    enum class mode {
        position  = 0,
        immediate = 1,
        relative  = 2,
    };

    struct instruction {
        opcode op;
        mode m1;
        mode m2;
        mode m3;
    };

    static instruction decode(std::int64_t instr) {
        instruction ir;
        ir.op = static_cast< opcode >(10 * (instr / 10 % 10) + instr % 10);
        ir.m1 = static_cast< mode >(instr / 100   % 10);
        ir.m2 = static_cast< mode >(instr / 1000  % 10);
        ir.m3 = static_cast< mode >(instr / 10000 % 10);
        return ir;
    }

    std::int64_t addr(std::int64_t pc, mode m) {
        switch (m) {
            case mode::position:
                return this->mem[pc];

            case mode::immediate:
                return pc;

            case mode::relative:
                return this->mem[pc] + this->base;

            default:
                throw std::invalid_argument(
                    "invalid mode " + std::to_string(static_cast< int >(m))
                );
        }
    }

    std::int64_t exec(std::int64_t pc, std::vector< std::int64_t >& in, std::vector< std::int64_t >& out) {
        while (true) {
            const auto ir = decode(this->mem[pc]);

            switch (ir.op) {
                case opcode::halt:
                    return -1;

                case opcode::add: {
                    const auto op1 = this->mem[this->addr(pc + 1, ir.m1)];
                    const auto op2 = this->mem[this->addr(pc + 2, ir.m2)];
                    const auto dst = this->addr(pc + 3, ir.m3);
                    this->mem[dst] = op1 + op2;
                    pc += 4;
                    break;
                }

                case opcode::mul: {
                    const auto op1 = this->mem[this->addr(pc + 1, ir.m1)];
                    const auto op2 = this->mem[this->addr(pc + 2, ir.m2)];
                    const auto dst = this->addr(pc + 3, ir.m3);
                    this->mem[dst] = op1 * op2;
                    pc += 4;
                    break;
                }

                case opcode::get: {
                    if (in.empty())
                        return pc;
                    const auto dst = this->addr(pc + 1, ir.m1);
                    this->mem[dst] = in.back();
                    in.pop_back();
                    pc += 2;
                    break;
                }

                case opcode::put: {
                    const auto val = this->mem[this->addr(pc + 1, ir.m1)];
                    out.push_back(val);
                    pc += 2;
                    break;
                }

                case opcode::jmp: {
                    const auto op1 = this->mem[this->addr(pc + 1, ir.m1)];
                    const auto op2 = this->mem[this->addr(pc + 2, ir.m2)];
                    pc = op1 ? op2 : pc + 3;
                    break;
                }

                case opcode::jmpz: {
                    const auto op1 = this->mem[this->addr(pc + 1, ir.m1)];
                    const auto op2 = this->mem[this->addr(pc + 2, ir.m2)];
                    pc = not op1 ? op2 : pc + 3;
                    break;
                }

                case opcode::less: {
                    const auto op1 = this->mem[this->addr(pc + 1, ir.m1)];
                    const auto op2 = this->mem[this->addr(pc + 2, ir.m2)];
                    const auto dst = this->addr(pc + 3, ir.m3);
                    this->mem[dst] = op1 < op2 ? 1 : 0;
                    pc += 4;
                    break;
                }

                case opcode::eq: {
                    const auto op1 = this->mem[this->addr(pc + 1, ir.m1)];
                    const auto op2 = this->mem[this->addr(pc + 2, ir.m2)];
                    const auto dst = this->addr(pc + 3, ir.m3);
                    this->mem[dst] = op1 == op2 ? 1 : 0;
                    pc += 4;
                    break;
                }

                case opcode::rela: {
                    this->base += this->mem[this->addr(pc + 1, ir.m1)];
                    pc += 2;
                    break;
                }

                default: {
                    const auto msg = "invalid opcode "
                                   + std::to_string(static_cast< std::int64_t >(ir.op));
                    throw std::invalid_argument(msg);
                }
            }
        }
    }

    intcode(const std::vector< std::int64_t >& program) {
        for (std::size_t i = 0; i < program.size(); ++i)
            this->mem[i] = program[i];
    }

private:
    std::unordered_map< std::int64_t, std::int64_t > mem;
    std::int64_t base = 0;
};

int main() {
    std::vector< std::int64_t > program;
    for (std::string line; std::getline(std::cin, line, ',');)
        program.push_back(std::stoll(line));

    auto ic = intcode(program);
    std::vector< std::int64_t > input = { 1 };
    std::vector< std::int64_t > output;
    ic.exec(0, input, output);

    for (auto o : output)
        std::cout << o << "\n";

    ic = intcode(program);
    input.clear();
    input.push_back(2);
    output.clear();
    ic.exec(0, input, output);

    for (auto o : output)
        std::cout << o << "\n";
}
