import os

os.chdir("..")
data = open(os.getcwd()+"/input.txt", "r").read().split("\n\n")


def crate() -> list[list[str]]:
    box = []
    box.append([i for i in "FLMW"[::-1]])
    box.append([i for i in "FMVZB"[::-1]])
    box.append([i for i in "QLSRVH"[::-1]])
    box.append([i for i in "JTMPQVSF"[::-1]])
    box.append([i for i in "WSL"[::-1]])
    box.append([i for i in "WJRMPVF"[::-1]])
    box.append([i for i in "FRNPCQJ"[::-1]])
    box.append([i for i in "BRWZSPHV"[::-1]])
    box.append([i for i in "WZHGCJMB"[::-1]])
    return box


def instruction(data: str):
    return [[int(j) for j in i.split(" ") if j.isdigit()] for i in data.split("\n")]


def main():
    inst = instruction(data[1])
    crt = crate()
    for i in inst:
        for j in range(i[0]):
            crt[i[2]-1].append(crt[i[1]-1].pop())
    print("stage 1 = ", "".join([i[-1] for i in crt]))
    crt2 = crate()
    for i in inst:
        crt2[i[2]-1].extend(crt2[i[1]-1][-(i[0]):])
        crt2[i[1]-1] = crt2[i[1]-1][:-(i[0])]
    print("stage 2 = ", "".join([i[-1] for i in crt2]))


print(main())
