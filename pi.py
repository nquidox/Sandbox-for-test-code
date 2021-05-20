print("Calculating Pi number...")

i = 0
sum = 0
sign = 1
precision = 100000000*2

for i in range(0, precision+1):
	D = (sign * (1 / (2 * i + 1)))
	sum = sum + D
	sign = -sign
	pi = 4 * sum

print(i, "  ", pi, "  ", D)
