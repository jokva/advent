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

extern "C" {
    #include <lua.h>
}

#include <luabind/luabind.hpp>
#include <luabind/object.hpp>

namespace {

std::vector< std::int64_t > vector(luabind::object arr) {
    auto xs = std::vector< int64_t >();
    auto fst = luabind::iterator(arr);
    auto end = luabind::iterator();
    std::transform(
        fst, end, std::back_inserter(xs),
        [](auto x) { return luabind::object_cast< std::int64_t >(x); }
    );

    return xs;
}

intcode mkintcode(luabind::object program) {
    return intcode(vector(program));
}

luabind::object eval(intcode* ic, luabind::object state) {
    auto pc = luabind::object_cast< int >(state["pc"]);
    auto in = luabind::object_cast< int >(state["camera"]);
    auto input  = std::vector< std::int64_t >();
    auto output = std::vector< std::int64_t >();
    input.push_back(in);
    pc = ic->exec(pc, input, output);

    auto ret = luabind::newtable(state.interpreter());
    ret["pc"] = pc;
    ret["motor"] = luabind::newtable(state.interpreter());
    ret["motor"]["colour"] = output[0];
    ret["motor"]["turn"]   = output[1];
    return ret;
}

}

extern "C" int luaopen_intcode(lua_State* L) {
    using namespace luabind;

    open(L);
    module(L) [
        class_< intcode >("intcode")
            .def("exec", &eval)
        ,
        def("load_program", mkintcode)

    ];

    return 0;
}

