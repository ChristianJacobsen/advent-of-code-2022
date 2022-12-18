from itertools import cycle
import sys


def get_rock(rock_number: int, y: int):
    match rock_number % 5:
        case 0:
            return set([(2, y), (3, y), (4, y), (5, y)])
        case 1:
            return set([(3, y + 2), (2, y + 1), (3, y + 1), (4, y + 1), (3, y)])
        case 2:
            return set([(4, y + 2), (4, y + 1), (2, y), (3, y), (4, y)])
        case 3:
            return set([(2, y + 3), (2, y + 2), (2, y + 1), (2, y)])
        case 4:
            return set([(2, y + 1), (3, y + 1), (2, y), (3, y)])
        case _:
            assert False


def move_left(rock: set[tuple[int, int]]):
    if any([x == 0 for (x, _) in rock]):
        return rock
    return set([(x - 1, y) for (x, y) in rock])


def move_right(rock: set[tuple[int, int]]):
    if any([x == 6 for (x, _) in rock]):
        return rock
    return set([(x + 1, y) for (x, y) in rock])


def move_down(rock: set[tuple[int, int]]):
    return set([(x, y - 1) for (x, y) in rock])


def move_up(rock: set[tuple[int, int]]):
    return set([(x, y + 1) for (x, y) in rock])


with open(sys.argv[1]) as f:
    gas_flow_direction = [*f.readline().strip()]

rock_positions = set([(x, int(0)) for x in range(7)])
tallest_rock_position = 0


def keep_top_n(n: int):
    for rock in [(x, y) for (x, y) in rock_positions if y <= tallest_rock_position - n]:
        rock_positions.remove(rock)


direction_iter = cycle(enumerate(gas_flow_direction))
offset = 0
rock_number = 0
seen_patterns: dict[tuple[int, tuple[tuple[int, int], ...]], tuple[int, int]] = {}
target_rock_number = 1000000000000

part_1 = 0
part_2 = 0

while rock_number < target_rock_number:
    keep_top_n(50)

    rock = get_rock(rock_number, tallest_rock_position + 4)

    while True:
        (i, direction) = next(direction_iter)

        match direction:
            case "<":
                rock = move_left(rock)
                if rock & rock_positions:
                    rock = move_right(rock)
            case ">":
                rock = move_right(rock)
                if rock & rock_positions:
                    rock = move_left(rock)
            case _:
                assert False

        rock = move_down(rock)
        if rock & rock_positions:
            rock = move_up(rock)
            rock_positions |= rock
            tallest_rock_position = max([y for (_, y) in rock_positions])

            state = tuple(sorted([(x, tallest_rock_position - y) for (x, y) in rock_positions]))
            key = (i, state)
            if key in seen_patterns:
                (prev_rock_number, prev_tallest) = seen_patterns[key]
                rock_number_diff = rock_number - prev_rock_number
                tallest_rock_diff = tallest_rock_position - prev_tallest
                amount = (target_rock_number - rock_number) // rock_number_diff
                offset += amount * tallest_rock_diff
                rock_number += amount * rock_number_diff

            seen_patterns[key] = (rock_number, tallest_rock_position)
            break

    rock_number += 1

part_2 = tallest_rock_position + offset

print(f"Part 2: {part_2}")
