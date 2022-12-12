import os
os.chdir("..")
data = open(os.getcwd()+"/input.txt", "r").read().split("\n")


def parse(i: str):
    match i:
        case "A": return 1
        case "B": return 2
        case "C": return 3
        case "X": return 1
        case "Y": return 2
        case "Z": return 3


def score(x: int, y: int):
    if x == y:
        return x+3
    elif x == y+1 or x == y-2:
        return x+6
    else:
        return x


def guess(x: int, y: int):
    if x == 2:
        return score(y, y)
    elif x == 1:
        if y == 1:
            return score(3, 1)
        else:
            return score(y-1, y)
    else:
        if y == 3:
            return score(1, 3)
        else:
            return score(y+1, y)


def main(data: list[str]):
    stage1 = sum([score(parse(i.split(" ")[1]), parse(i.split(" ")[0]))
                 for i in data])
    stage2 = sum([guess(parse(i.split(" ")[1]), parse(i.split(" ")[0]))
                 for i in data])
    print("stage 1 = ", str(stage1))
    print("stage 2 = ", str(stage2))


main(data)
