ans = [0, 0, 0]
line = input()
while len(line) > 0:
    # New elf
    tot = 0
    while len(line) > 0:
        tot += int(line)
        line = input()
    
    i = ans.index(min(ans))
    if ans[i] < tot:
        ans[i] = tot
    line = input()

print(sum(ans))