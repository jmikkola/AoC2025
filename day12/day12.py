import sys


def main(args):
    fname = 'example'
    if args:
        fname = args[0]

    with open(fname) as f:
        lines = [l.strip() for l in f.readlines()]

    parsed = parse(lines)
    print('part 1:', part1(*parsed))


def part1(shapes, problems):
    total = 0
    for problem in problems:
        if solvable(shapes, *problem):
            total += 1
    return total


def solvable(shapes, dimensions, counts):
    x, y = dimensions
    area = x * y

    area_needed = 0
    for i in range(len(counts)):
        shape = shapes[i]
        count = counts[i]
        area_needed += count * area_of(shape)
    print(area_needed, area)
    if area_needed > area:
        return False

    return True


def area_of(shape):
    total = 0
    for line in shape:
        for cell in line:
            if cell == '#':
                total += 1
    return total


def parse(lines):
    shapes = []
    shape = []
    problems = []

    for line in lines:
        if 'x' in line:
            size, rest = line.split(':')
            x, y = size.split('x')
            counts = [int(n) for n in rest.strip().split(' ')]
            problems.append(((int(x), int(y)), counts))
        elif ':' in line:
            continue
        elif not line:
            if shape:
                shapes.append(shape)
                shape = []
        else:
            shape.append(line)

    return (shapes, problems)


if __name__ == '__main__':
    main(sys.argv[1:])
