import sys


def read_first_arg():
    input_str = "No input given" if len(sys.argv) <= 1 else sys.argv[1]
    print(f"The first argument provided is: {input_str}.")


if __name__ == "__main__":
    read_first_arg()
