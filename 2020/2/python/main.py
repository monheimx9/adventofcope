import re

v_low = 0
v_max = 0
v_let = ''
pwd = ''
totalok = 0
totalok2 = 0


with open('input.txt', 'r') as file:
    pattern = r"(\d+)-(\d+)\s(\w):\s(\w+)"
    for line in file:
        match = re.search(pattern, line)

        if match:
            v_low = int(match.group(1))
            v_max = int(match.group(2))
            v_let = match.group(3)
            pwd = match.group(4)
            count_v_let = pwd.count(v_let)
            if count_v_let >= v_low and count_v_let <= v_max:
                totalok = totalok + 1
                #print(totalok)
            if pwd[v_low - 1] == v_let or pwd[v_max - 1] == v_let:
                if not pwd[v_low - 1] == pwd[v_max - 1]:
                    totalok2 = totalok2 + 1
                    print(totalok2)
            
