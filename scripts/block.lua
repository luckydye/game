function update(dt)
    v = math.sin(dt / 100) / 80;
    rotate(v)
end

update(time())
