require('intcode')

function getinput()
    local output = {}
    local input = io.read('*line')
    for match in input:gmatch("(-?%d+),?") do
        output[#output + 1] = tonumber(match)
    end
    return output
end

function rotate(dir, turn)
    local left = 0
    local right = 1

    -- direction faced after turning
    local directions = {
        [left] = {
            ['up']    = 'left',
            ['left']  = 'down',
            ['down']  = 'right',
            ['right'] = 'up',
        },
        [right] = {
            ['up']    = 'right',
            ['right'] = 'down',
            ['down']  = 'left',
            ['left']  = 'up',
        },
    }

    return directions[turn][dir]
end

function move(pos, dir)
    -- coordinate changes for the turned-to direction
    -- up is -1, since we'll print from the top
    local moves = {
        ['up']    = { 0, -1},
        ['down']  = { 0,  1},
        ['left']  = {-1,  0},
        ['right'] = { 1,  0},
    }

    -- move 1 panel in facing direction
    local mv = moves[dir]
    return {
        pos[1] + mv[1],
        pos[2] + mv[2],
    }
end

function pairkey (k)
    -- can't use tables (pairs, really) for keys, so just stringify it.
    -- probably better to use nested tables, since they're ergonomic in lua
    return string.format("%d,%d", k[1], k[2])
end

local black = 0
local white = 1

function paintjob(program, firstcolour)
    local channel = {
        ['pc'] = 0
    }

    local robot = {
        ['hull'] = { [pairkey({0, 0})] = firstcolour },
        ['pos']  = {0, 0},
        ['dir']  = 'up'
    }

    ic = load_program(program)
    while true do
        -- read tile colour, default to black
        channel['camera'] = robot['hull'][pairkey(robot['pos'])]
        if channel['camera'] == nil then
            channel['camera'] = black
        end

        -- run intcode
        channel = ic:exec(channel)

        -- halt, so no output
        if channel['pc'] == -1 then break end

        if channel['motor']['colour'] > 1 then
            error('invalid colour')
        end
        if channel['motor']['turn'] > 1 then
            error('invalid colour')
        end

        local colour = channel['motor']['colour']
        -- paint the current tile
        robot['hull'][pairkey(robot['pos'])] = colour

        -- rotate & move
        local turn = channel['motor']['turn']
        robot['dir'] = rotate(robot['dir'], turn)
        robot['pos'] = move(robot['pos'], robot['dir'])
    end

    return robot['hull']
end

function tablelength(T)
    local count = 0
    for _ in pairs(T) do count = count + 1 end
    return count
end

function boundbox(T)
    local minimum = { 0, 0 }
    local maximum = { 0, 0 }

    for str in pairs(T) do
        for x, y in str:gmatch("(-?%d+),(-?%d+)") do
            x = tonumber(x)
            y = tonumber(y)
            minimum[1] = math.min(minimum[1], x)
            minimum[2] = math.min(minimum[2], y)

            maximum[1] = math.max(maximum[1], x)
            maximum[2] = math.max(maximum[2], y)
        end
    end

    return {
        ['min'] = minimum,
        ['max'] = maximum,
    }
end

local input = getinput()
local first = paintjob(input, black)
print('panels covered at least once:', tablelength(first))

local second = paintjob(input, white)
local box = boundbox(second)
local hull = {}
for y = box['min'][2], box['max'][2] do
    s = ''
    if not hull[y] then hull[y] = {} end
    for x = box['min'][1], box['max'][1] do
        key = pairkey({x, y})
        s = s .. (second[key] == white and '#' or ' ')
    end
    print(s)
end
