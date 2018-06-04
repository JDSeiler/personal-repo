'''bubble sort in python'''

def bubble(array):
	sort = False
	while sort == False:
		swap = 0
		for num in array:
			if array.index(num) == len(array) - 1:
				pass
			elif array[array.index(num)] > array[array.index(num) + 1]:
				theIndex = array.index(num)
				theBubble = array.pop(theIndex)
				array.insert(theIndex + 1, theBubble)
				swap += 1
			else:
				pass
		if swap == 0:
			sort = True
			return array


