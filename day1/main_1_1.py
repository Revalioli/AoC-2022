ans = 0
line = input()
while len(line) > 0:
    # New elf
    tot = 0
    while len(line) > 0:
        tot += int(line)
        line = input()
    ans = max(tot, ans)
    line = input()

print(ans)