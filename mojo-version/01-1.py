
file = open("day-1.txt", "r")
data = file.read()
lines = data.splitlines()
list_a =  []
list_b = []
for i in range(len(lines)):
    el = lines[i]
    x = el.split("   ")
    list_a.append(int(x[0]))
    list_b.append(int(x[1]))

list_a.sort()
list_b.sort()
sum = 0
for i in range(len(list_a)):

    a = list_a[i]
    b = list_b[i]

    if a > b:
        sum = sum + (a-b)
    else:
        sum = sum + (b-a)
print(sum)
