import random
import sys
import math
import distances


def distance_squared(p1, p2):
    dx = p2[0] - p1[0]
    dy = p2[1] - p1[1]
    return dx * dx + dy * dy


# this is not an efficient algorithm to find distances to nearest neighbors
# but we use it here to keep things relatively simple
def nearest_distance(reference_point, other_points) -> float:
    min_distance = sys.float_info.max
    for point in other_points:
        d2 = distance_squared(reference_point, point)
        # make sure we don't locate the point itself otherwise minimum
        # distance is zero
        if d2 > sys.float_info.epsilon:
            min_distance = min(min_distance, d2)
    return math.sqrt(min_distance)


def generate_random_points(num_points, extent):
    return [
        (random.uniform(-extent, extent), random.uniform(-extent, extent))
        for _ in range(num_points)
    ]


if __name__ == "__main__":
    points = generate_random_points(num_points=5000, extent=1.0)

    ds_python = list(map(lambda x: nearest_distance(x, points), points))

    ds_rust = distances.nearest_distances(points)

    for (d_python, d_rust) in zip(ds_python, ds_rust):
        print(d_python, d_rust)
