import os

os.chdir("..")
data = open(os.getcwd()+"/input.txt", "r").read().split("\n")

print(data)
