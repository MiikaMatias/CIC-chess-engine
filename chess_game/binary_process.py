
def help():
    binary_representation = "00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000".replace(" ", "")
    decimal_number = int(binary_representation.replace(" ", ""), 2)
    print(len(binary_representation))
    print(binary_representation)
    print(decimal_number)

if __name__ == "__main__":
    help()