import time

from functools import cache

def fib_uc(n):
    if n < 2:
        return n
    else:
        return fib_uc(n - 1) + fib_uc(n - 2)

fib_c = cache(fib_uc)

def fib_dp(n):

    l = [0, 1]

    for i in range(0, n - 1):
        
        l[0], l[1] = l[1], l[0] + l[1]

    return l[1]

for i in range(0, 50):

    start = time.time()

    r = fib_dp(i)

    end = time.time()

    print(i, r, end - start)