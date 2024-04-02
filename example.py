from distances import nearest_distances

from random import random


points1 = [(random(), random()) for _ in range(20)]
points2 = [(random(), random()) for _ in range(20)]

distances = nearest_distances(points1, points2)

print(distances)


# also try:
# help(nearest_distances)
