elves = IO.read('day1.input').split("\n\n").map{|e| e.split("\n").map(&:to_i).sum}
puts "#1: #{elves.max}"
puts "#2: #{elves.max(3).sum}"
