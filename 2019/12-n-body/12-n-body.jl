struct Moon
    x::Int
    y::Int
    z::Int

    vx::Int
    vy::Int
    vz::Int
end

Moon(x, y, z) = Moon(x, y, z, 0, 0, 0)

Base.show(io::IO, m::Moon) = begin
    pos = "pos=<x=$(m.x), y=$(m.y), z=$(m.z)>"
    vel = "vel=<x=$(m.vx), y=$(m.vy), z=$(m.vz)>"
    print(io, pos * ", " * vel)
end

function energy(m::Moon)
    pot = sum(abs(x) for x in (m.x, m.y, m.z))
    kin = sum(abs(x) for x in (m.vx, m.vy, m.vz))
    return pot * kin
end

function Moon(str::AbstractString)
    # <...> => ...
    a = str[2:end-1]
    x, y, z = split(a, ", ")
    Moon(
        parse(Int, x[3:end]),
        parse(Int, y[3:end]),
        parse(Int, z[3:end]),
    )
end

# function pull(moon::Moon, moons)
function pull(moon, moons)
    function pulld(lhs, rhs)
        if lhs < rhs return  1 end
        if rhs < lhs return -1 end
        return 0
    end

    xs = (pulld(moon.x, m.x) for m in moons)
    ys = (pulld(moon.y, m.y) for m in moons)
    zs = (pulld(moon.z, m.z) for m in moons)

    return Moon(
        moon.x,
        moon.y,
        moon.z,
        moon.vx + sum(xs),
        moon.vy + sum(ys),
        moon.vz + sum(zs),
    )
end

function move(m::Moon)
    return Moon(
        m.x + m.vx,
        m.y + m.vy,
        m.z + m.vz,
        m.vx,
        m.vy,
        m.vz,
    )
end

function step(moons::Array{Moon})
    gravity = (pull(moon, moons) for moon in moons)
    return [move(moon) for moon in gravity]
end

function iterate(f, init, n)
    x = init
    for _ in 1:n
        x = f(x)
    end
    return x
end

function getx(m) return m.x end
function gety(m) return m.y end
function getz(m) return m.z end

function getvx(m) return m.vx end
function getvy(m) return m.vy end
function getvz(m) return m.vz end

function cycle_periods(moons, getpos, getvel)
    # find the cycle period for each dimension
    # first repeated element will necessarily be the initial state
    pos = [getpos(m) for m in moons]
    vel = [0 for _ in moons]

    i = 1
    while true
        moons = step(moons)
        ps = [getpos(m) for m in moons]
        vs = [getvel(m) for m in moons]
        if ps == pos && vs == vel
            return i
        end
        i += 1
    end
end

function main()
    moons = [Moon(strip(ln)) for ln in readlines()]
    step1000 = iterate(step, moons, 1000)
    println("energy after 1000 steps: ", sum(map(energy, step1000)))

    cyclex = cycle_periods(moons, getx, getvx)
    cycley = cycle_periods(moons, gety, getvy)
    cyclez = cycle_periods(moons, getz, getvz)
    println("cycles after ", lcm(cyclex, cycley, cyclez))
end

main()
