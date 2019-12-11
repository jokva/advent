img = ARGF.readline.strip
layersize = 25 * 6
layers = img.each_char.each_slice(layersize).to_a

fewest0 = layers.map {|x| [x.count('0'), x]}.min_by(&:first).last
puts 'checksum: ', fewest0.count('1') * fewest0.count('2')

def select_layer(over, current)
  if over == '2' # transparent, so just take whatever's underneath
    return current
  else # colour already set, so keep it
    return over
  end
end

msg = layers.reduce {|accu, l| accu.zip(l).map {|a,b|select_layer(a, b)}}
puts msg.map {|x| x == '0' ? ' ' : '#' }.each_slice(25).to_a.map(&:join)
