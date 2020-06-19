n = int(input())
r = 0
for p in range(2, int(n**0.5)+1):
    e = 0
    while n%p == 0:
        e += 1
        n /= p
    i = 1
    while e >= i:
        e -= i
        r += 1
        i += 1
if n != 1:
    r += 1
print(r)
