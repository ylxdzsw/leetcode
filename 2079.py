from typing import List

def wateringPlants(plants: List[int], capacity: int) -> int:
    steps = len(plants)
    remaining_water = capacity
    for next_plant_index in range(len(plants)):
        if remaining_water < plants[next_plant_index]:
            remaining_water = capacity
            steps += 2 * next_plant_index
        remaining_water -= plants[next_plant_index]
    return steps

assert wateringPlants([2,2,3,3], 5) == 14
assert wateringPlants([1,1,1,4,2,3], 4) == 30
assert wateringPlants([7,7,7,7,7,7,7], 8) == 49
