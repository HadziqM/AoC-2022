import os

os.chdir("..")
data = open(os.getcwd()+"/input.txt", "r").read().split("\n")


def input(data: list[str]):
    return [[[int(k) for k in j.split("-")]
             for j in i.split(",")] for i in data]


def main(data: list[str]):
    data_input = input(data)
    stage1 = 0
    for i in data_input:
        if i[0][0] in range(i[1][0], i[1][1]+1) and i[0][1] in range(i[1][0], i[1][1]+1):
            stage1 += 1
            continue
        if i[1][0] in range(i[0][0], i[0][1]+1) and i[1][1] in range(i[0][0], i[0][1]+1):
            stage1 += 1
    stage2 = len([i for i in data_input if len([j for j in range(i[0][0], i[0][1]+1) if j in [k for k in range(i[1][0], i[1][1]+1)]])
                 != 0])
    print("stage 1 = ", stage1)
    print("stage 2 = ", stage2)


main(data)
