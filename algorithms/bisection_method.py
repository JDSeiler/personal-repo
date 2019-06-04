'''For performing the bisection algorithm on function f(x)'''

import math


def f(x):
    a = math.pow(x, 4)
    b = 2 * (math.pow(x, 3))
    c = math.pow(x, 2)
    result = (a - b - c)
    return result


x = 0
while x != 1:
    print("Point A must be less than point B")
    A = float(input("Choose your first value: "))
    B = float(input("Choose youre second value: "))
    if A < B and f(A) < 0 and f(B) > 0 or f(A) > 0 and f(B) < 0:
        x = 1
    else:
        print("Inputs wont work, try different values.")


max_iteration = 50
count = 0
tolerance = .001

while count < max_iteration:
    C = (A + B) / 2
    if f(C) == 0:
        print(f"{C} is your root")
        break
    if (C - A) <= tolerance:
        print(f"{C} is your root")
        break
    if count > max_iteration:
        print("Failed")
        break
    elif f(A) > 0 and f(C) > 0:
        A = C
        count += 1
    elif f(A) < 0 and f(C) < 0:
        A = C
        count += 1
    elif f(B) > 0 and f(C) > 0:
        B = C
        count += 1
    elif f(B) < 0 and f(C) < 0:
        B = C
        count = + 1
