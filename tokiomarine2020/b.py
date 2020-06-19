A, V = map(int, input().split())
B, W = map(int, input().split())
T = int(input())

if V == W:
    print('NO')
    exit()

if 0 <= (abs(B - A) / (V - W)) <= T:
    print('YES')
else:
    print('NO')
