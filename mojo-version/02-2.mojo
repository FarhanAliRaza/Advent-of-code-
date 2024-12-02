


def is_incsreasing(li:  List[Int]) -> Bool:
    l = 0
    r = 1
    var bs = List[Bool]()
    while r < len(li):
        if li[r] > li[l]:
            bs.append(True)
        else:
            bs.append(False)
        l += 1
        r += 1
    true_count = bs.count(True)
    false_count = bs.count(False)

    return true_count > false_count
    

def is_valid(li: List[Int]) -> Tuple[Bool, List[Int]]:
    l = 0
    r = 1

    is_inc = is_incsreasing(li)
    
    while r < len(li):
        if is_inc:
            diff   = li[r] - li[l]
            if not (diff >= 1 and diff <= 3):
                if (r == len(li) - 1):
                    _ = li.pop(r);

                elif (li[r+1] -  li[l] <= 3) and (li[r+1] -  li[l] >= 1):
                    _ =li.pop(r)

                else:
                    _ = li.pop(l); 
                
                return  False,li ;
        else:
            diff = li[l] - li[r]
            if not (diff <= 3 and diff >= 1):
            
                if r == len(li) - 1:
                    _ =li.pop(r);
                
                elif li[l] -  li[r+1]  <= 3 and  li[l] -  li[r+1]  >= 1:
                    _ =li.pop(r);
                else:
                    _ =li.pop(l);
                
                return  False,li ;              

        l += 1
        r += 1
    return  True,li ; 







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
        res, li = is_valid(line)
        if res:
            safe += 1
        else:
            res, _ = is_valid(li)
            if res:
                safe += 1

    print(safe)






    


        

