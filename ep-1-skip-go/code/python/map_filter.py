def implementation():
    out = list(filter(lambda m: m % 3 == 0, map(lambda n: n * 2, range(10))))
    print("Map & Filter:", out)
