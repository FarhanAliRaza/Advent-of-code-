



def main():
    file = open("day-1.txt", "r")
    lines = file.read().splitlines()
    list_a =  List[Int]()
    list_b = List[Int]()
    for i in range(len(lines)):
        el = lines[i]
        x = el.split("   ")
        list_a.append(int(x[0]))
        list_b.append(int(x[1]))

    sort(list_a)
    sort(list_b)
    sum = 0
    for i in range(len(list_a)):

        a = list_a[i]

        sum = sum + a * list_b.count(a)
     
    print(sum)

    
    


        

