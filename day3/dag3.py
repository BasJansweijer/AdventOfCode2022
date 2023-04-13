items_file = open('input.input').read()


item_list = ['a', 'b', 'c', 'd', 'e', 'f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z', 'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z']
priority_list = [i for i in range(1,53)]
doubles = []

for i in items_file.split():
    index_1 = 0
    index_2 = int((len(i) / 2))
    index_3 = int(len(i))
    first_half = i[index_1 : index_2]
    second_half = i[index_2 : index_3] 
    for j in item_list:
        if j in first_half and j in second_half:
            doubles.append(j)

sum_priority = 0
for i in item_list:
    for k in doubles:
        if k == i:
            index_relevant = item_list.index(i)
            sum_priority += priority_list[index_relevant]
                
print(sum_priority)




        
