# Take a list of positive integers < 10:
#   * Multiply each integer by 3
#   * Filter any values that aren't divisible by 3

def implementation():
    out = list(
            filter(lambda n: n % 3 == 0,
                map(lambda m: m * 2, 
                    range(10)
                )
            )
        )

    print("Map & Filter Output:", out) # Expected output: [0, 6, 12, 18]
