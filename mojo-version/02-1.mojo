


def is_incsreasing(li:  List[Int]) -> Bool:
    l = 0
    r = 1
    if li[r] > li[l]:
        return True

    return False        




def is_valid(li: List[Int]) -> Bool:
    l = 0
    r = 1

    is_inc = is_incsreasing(li)
    while r < len(li):
        if is_inc:
            diff   = li[r] - li[l]
            if not (li[r] > li[l] and diff >= 1 and diff <= 3):
                return False
        else:
            diff = li[l] - li[r]
            if not (li[l] > li[r] and diff >= 1 and diff <= 3):
                return False
        l += 1
        r += 1
    return True







def main():
    var f = open("/home/farhan/code/advent_of_code/day_1/mojo-version/day-2.txt", "r").read()
    # print(f)
    var safe = 0

    lines = f.splitlines()

    for l in lines:
        digits = l[].split()
        var line = List[Int]()
        for d in digits:
            line.append(int(d[]))
 
        if is_valid(line):
            safe += 1
    print(safe)






    


        

