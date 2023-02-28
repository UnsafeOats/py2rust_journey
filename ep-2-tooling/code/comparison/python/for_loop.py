# Take a list of positive integers < 10:
#   * Multiply each integer by 3
#   * Filter any values that aren't divisible by 3

def implementation():
    a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    b = []

    for n in a:
        new_val = n * 2
        if new_val % 3 == 0:
            b.append(new_val)

    print("For Loop Output:", b) # Expected output: [0, 6, 12, 18]
