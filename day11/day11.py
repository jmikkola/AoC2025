import sys


def main(args):
    fname = 'example'
    if args:
        fname = args[0]

    with open(fname) as f:
        lines = [l.strip() for l in f.readlines()]

    parsed = parse(lines)
    print('part 1:', part1(parsed))


def parse(lines):
    result = {}
    for line in lines:
        key, rest = line.split(':')
        outputs = rest.strip().split(' ')
        result[key] = outputs
    return result


def part1(parsed):
    return dfs(parsed, 'you', set())


def dfs(graph, node, seen):
    if node == 'out':
        return 1

    seen.add(node)

    total = 0
    for connection in graph.get(node, []):
        if connection in seen:
            continue

        total += dfs(graph, connection, seen)

    seen.remove(node)
    return total


if __name__ == '__main__':
    main(sys.argv[1:])
