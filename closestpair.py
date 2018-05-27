'''This code is still work in progress, do not expect polish or documentation.'''



import math as m
from operator import itemgetter
 
testData = [[38, 32], [93, 72], [-20, -82], [31, -77], [-90, -29], [-58, -41], [44, 6], [-6, 15], [-56, 7], [-5, 85], [90, -42], [-5, 82], [-27, 14], [-74, -23], [71, -54],
            [-26, -18], [67, 58], [52, 100], [49, -75], [17, -35], [25, 54], [-45, -38], [-92, 25], [97, 95], [-63, -90], [92, -62], [-29, 4], [-41, -4], [-68, 84], [91, -2], 
            [8, -88], [-2, -87], [-57, -91], [75, -60], [-17, 13], [-61, -23], [97, -88], [89, 22], [-71, 39], [-50, -30], [11, -25], [90, -7], [42, -34], [92, 90], [52, -92], 
            [-79, 87], [-14, 19], [9, 10], [10, 25], [77, -82]]

def distanceB(point1, point2):
    x1 = point1[0]
    y1 = point1[1]
    x2 = point2[0]
    y2 = point2[1]
    dis = round(m.sqrt((((x2 - x1)**2) + ((y2 - y1)**2))), 6)
    return dis

xWise = sorted(testData, key = itemgetter(0))
yWise = sorted(testData, key = itemgetter(1))

xPass = []
yPass = []

tol = 3

for point in xWise:

    if xWise.index(point) == len(xWise) - 2:
        nextP =  xWise[xWise.index(point) + 1]
        if nextP[0] - point[0] <= tol:
            xPass.append((point, nextP))
            break
        else:
            break

    shift = 1
    nextP =  xWise[xWise.index(point) + shift]
    if nextP[0] - point[0] <= tol:
        check = True
        shift += 1
        xPass.append((point, nextP))
    else:
        check = False
    while check == True:
        nextP =  xWise[xWise.index(point) + shift]
        if nextP[0] - point[0] <= tol:
            shift += 1 
            xPass.append((point, nextP))
        else:
            check = False

for point in yWise:
    if yWise.index(point) == len(yWise) - 2:
        nextP =  yWise[yWise.index(point) + 1]
        if nextP[1] - point[1] <= tol:
            yPass.append((point, nextP))
            break
        else:
            break

    shift = 1
    nextP =  yWise[yWise.index(point) + shift]
    if nextP[1] - point[1] <= tol:
        check = True
        shift += 1
        yPass.append((point, nextP))
    else:
        check = False
    while check == True:
        nextP =  yWise[yWise.index(point) + shift]
        if nextP[1] - point[1] <= tol:
            shift += 1 
            yPass.append((point, nextP))
        else:
            check = False

passed = []
for pair in xPass:
    if pair in yPass:
        passed.append(pair)

print(passed)
