# 蟻本p.42　硬貨の問題
A = 620
C = (3, 2, 1, 3, 0, 2)
V = (1, 5, 10, 50, 100, 500)

def solve():
    ans = 0
    A = 620
    for i in reversed(range(6)):
        t = min(A // V[i], C[i])
        A -= t * V[i]
        ans += t
    return ans

print(solve())
