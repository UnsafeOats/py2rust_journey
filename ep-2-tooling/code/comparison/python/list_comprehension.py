# Take a list of positive integers < 10:
#   * Multiply each integer by 3
#   * Filter any values that aren't divisible by 3

def implementation():
    out = [n * 2 for n in range(10) if n % 3 == 0]

    print("List Comprehension Output:", out) # Expected output: [0, 6, 12, 18]
