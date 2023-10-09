#n_file = "input.txt"
#nbr_1 = 0
#nbr_2 = 0
#
#try:
#	with open(n_file, 'r') as file:
#		numlist = file.readlines()
#		for line in numlist:
#			nbr_1 = int(line.strip())
#			print(line.strip())
#			for line in numlist:
#				print(line.strip())
#				print (f'Le nbr1 = a {nbr_1}')
#				nbr_2 = int(line.strip())
#				if ((nbr_1) + (nbr_2)) == 2020:
#					print(f"{nbr_1} * {nbr_2} = {nbr_1 * nbr_2}")
 #                   exit




#except FileNotFoundError:
#	print("File not found :^(")
#except Exception as e:
#	print(f"Error {e}")



def recherche_somme_2020_3(liste):
    for i in range(len(liste)):
        for j in range(i + 1, len(liste)):
            for k in range(j + 1, len(liste)):
                if int(liste[i]) + int(liste[j]) + int(liste[k]) == 2020:
                    return liste[i], liste[j], liste[k]
    return None, None, None


file = open('input.txt', 'r')
numlist = file.readlines()

nombre1, nombre2, nombre3 = recherche_somme_2020_3(numlist)

if nombre1 is not None and nombre2 is not None and nombre3 is not None:
    produit = int(nombre1) * int(nombre2) * int(nombre3)
    print(f"Les nombres {nombre1}, {nombre2} et {nombre3} ont une somme de 2020.")
    print(f"Le produit de ces trois nombres est : {produit}")
else:
    print("Aucun triplet de nombres n'a une somme de 2020.")
