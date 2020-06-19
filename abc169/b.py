N=int(input())
A=list(map(int,input().split()))
K=1
if 0 in A:
    print(0)
else:
    for num in A:
        K*=num
        if K>10**18:
            print(-1)
            exit()
    print(K)
