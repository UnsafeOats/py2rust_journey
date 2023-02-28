# Using list comprehension

def implementation():
    out = [n * 2 for n in range(10) if (n * 2) % 3 == 0]
    print("List Comp:", out)
