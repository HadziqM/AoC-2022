import os
import string

os.chdir("..")
data = open(os.getcwd()+"/input.txt", "r").read().split("\n")


def get_score(i: str):
    return list(string.ascii_letters).index(i) + 1


def main(data: list[str]):
    stage1 = sum([[get_score(j) for j in i[:int(len(i)/2)]
                   if j in i[int(len(i)/2):]][0] for i in data])
    print("stage 1 = ", stage1)
    stage2 = []
    for i in range(int(len(data)/3)):
        first = [j for j in data[i*3] if j in data[i*3+1]]
        second = [j for j in first if j in data[i*3+2]][0]
        stage2.append(get_score(second))
    print("stage2 = ", sum(stage2))


main(data)
