#include <algorithm>
#include <ciso646>
#include <iostream>
#include <string>
#include <vector>

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
    };

    enum class mode {
        position  = 0,
        immediate = 1,
    };

    struct instruction {
        opcode op;
        mode m1;
        mode m2;
        mode m3;
    };

    static instruction decode(int instr) {
        instruction ir;
        ir.op = static_cast< opcode >(10 * (instr / 10 % 10) + instr % 10);
        ir.m1 = static_cast< mode >(instr / 100   % 10);
        ir.m2 = static_cast< mode >(instr / 1000  % 10);
        ir.m3 = static_cast< mode >(instr / 10000 % 10);
        return ir;
    }

    int read(int a, mode m) const {
        switch (m) {
            case mode::position:
                return this->mem.at(a);

            case mode::immediate:
                return a;

            default:
                throw std::invalid_argument(
                    "invalid mode " + std::to_string(static_cast< int >(m))
                );
        }
    }

    int exec(int pc, std::vector< int >& in, std::vector< int >& out) {
        while (true) {
            const auto ir = decode(this->mem.at(pc));

            switch (ir.op) {
                case opcode::halt:
                    return -1;

                case opcode::add: {
                    const auto op1 = this->mem.at(pc + 1);
                    const auto op2 = this->mem.at(pc + 2);
                    const auto dst = this->mem.at(pc + 3);
                    this->mem.at(dst) = this->read(op1, ir.m1)
                                      + this->read(op2, ir.m2);
                    pc += 4;
                    break;
                }

                case opcode::mul: {
                    const auto op1 = this->mem.at(pc + 1);
                    const auto op2 = this->mem.at(pc + 2);
                    const auto dst = this->mem.at(pc + 3);
                    this->mem.at(dst) = this->read(op1, ir.m1)
                                      * this->read(op2, ir.m2);
                    pc += 4;
                    break;
                }

                case opcode::get: {
                    if (in.empty())
                        return pc;
                    const auto dst = this->mem.at(pc + 1);
                    this->mem.at(dst) = in.back();
                    in.pop_back();
                    pc += 2;
                    break;
                }

                case opcode::put: {
                    const auto src = this->mem.at(pc + 1);
                    const auto val = this->read(src, ir.m1);
                    out.push_back(val);
                    pc += 2;
                    break;
                }

                case opcode::jmp: {
                    const auto op1 = this->mem.at(pc + 1);
                    const auto op2 = this->mem.at(pc + 2);
                    if (this->read(op1, ir.m1))
                        pc = this->read(op2, ir.m2);
                    else
                        pc += 3;
                    break;
                }

                case opcode::jmpz: {
                    const auto op1 = this->mem.at(pc + 1);
                    const auto op2 = this->mem.at(pc + 2);
                    if (not this->read(op1, ir.m1))
                        pc = this->read(op2, ir.m2);
                    else
                        pc += 3;
                    break;
                }

                case opcode::less: {
                    const auto op1 = this->mem.at(pc + 1);
                    const auto op2 = this->mem.at(pc + 2);
                    const auto dst = this->mem.at(pc + 3);
                    const auto lhs = this->read(op1, ir.m1);
                    const auto rhs = this->read(op2, ir.m2);
                    this->mem.at(dst) = lhs < rhs ? 1 : 0;
                    pc += 4;
                    break;
                }

                case opcode::eq: {
                    const auto op1 = this->mem.at(pc + 1);
                    const auto op2 = this->mem.at(pc + 2);
                    const auto dst = this->mem.at(pc + 3);
                    const auto lhs = this->read(op1, ir.m1);
                    const auto rhs = this->read(op2, ir.m2);
                    this->mem.at(dst) = lhs == rhs ? 1 : 0;
                    pc += 4;
                    break;
                }

                default: {
                    const auto msg =  "invalid opcode "
                                   + std::to_string(static_cast< int >(ir.op));
                    throw std::invalid_argument(msg);
                }
            }
        }
    }

    intcode(std::vector< int > memory) : mem(std::move(memory)) {}

private:
    std::vector< int > mem;
};

void p1(const std::vector< int >& input) {
    auto phases = std::vector< int > { 0, 1, 2, 3, 4 };

    int highest = -1;
    do {
        auto in  = std::vector< int >();
        auto out = std::vector< int >();
        auto ic  = intcode(input);
        in.push_back(0);
        for (auto phase : phases) {
            out.clear();
            in.push_back(phase);
            ic.exec(0, in, out);
            in = out;
        }

        highest = std::max(highest, out.back());
    } while (std::next_permutation(phases.begin(), phases.end()));

    std::cout << "maximum thrust: " << highest << "\n";
}

void p2(const std::vector< int >& input) {
    auto phases = std::vector< int > { 5, 6, 7, 8, 9 };

    int highest = -1;
    do {
        std::vector< std::vector< int > > ios(5);
        std::vector< int > pcs(5, 0);

        std::vector< intcode > ics = {
            intcode(input),
            intcode(input),
            intcode(input),
            intcode(input),
            intcode(input),
        };

        ios.back().push_back(0);
        for (std::size_t i = 0; i < phases.size(); ++i)
            ios[i].push_back(phases[i]);

        while (true) {
            for (std::size_t i = 0; i < ics.size(); ++i) {
                if (pcs[i] == -1) continue;
                pcs[i] = ics[i].exec(pcs[i], ios[i], ios[(i + 1) % 5]);
            }

            auto alive = [](int x) { return x != -1; };
            if (std::find_if(pcs.begin(), pcs.end(), alive) == pcs.end())
                break;
        }

        highest = std::max(highest, ios.back().back());
    } while (std::next_permutation(phases.begin(), phases.end()));

    std::cout << "maximum thrust (with feedback): " << highest << "\n";
}

int main() {
    std::string line;
    std::vector< int > input;
    for (std::string line; std::getline(std::cin, line, ',');)
        input.push_back(std::stoi(line));

    p1(input);
    p2(input);
}
