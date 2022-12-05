import os
os.chdir("..")
path = os.getcwd()
data = open(path+"/input.txt", "r")


def calories(data: str):
    elves = data.split("\n\n")
    idk = []
    for i in elves:
        idk.append(sum([int(j) for j in i.split("\n")]))
    return idk


def stage1(list: list):
    highest = list[0]
    for i in list:
        if i > highest:
            highest = i
    return highest


def stage2(list: list):
    vector = []
    for i in range(3):
        highest = stage1(list)
        vector.append(highest)
        list.remove(highest)
    return sum(vector)


parsed = calories(data.read())
data.close()
print("stage 1 = "+str(stage1(parsed)))
print("stage 2 = "+str(stage2(parsed)))
