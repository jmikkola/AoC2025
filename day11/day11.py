import sys
from collections import defaultdict


def main(args):
    fname = 'example'
    if args:
        fname = args[0]

    with open(fname) as f:
        lines = [l.strip() for l in f.readlines()]

    parsed = parse(lines)
    print('part 1:', part1(parsed))
    print('part 2:', part2(parsed))


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


def part2(graph):
    total = 0

    q = [('svr', 1, 0, 0, 0)]

    # The 0th iteration starts with paths of length 0, and the nth iteration
    # starts at points reached with paths of length n
    while q:
        q2 = []
        for (node, neither, fft, dac, both) in q:
            for neighbor in graph.get(node, []):
                if neighbor == 'out':
                    total += both
                    continue

                if neighbor == 'fft':
                    q2.append((neighbor, 0, neither, 0, dac))
                elif neighbor == 'dac':
                    q2.append((neighbor, 0, 0, neither, fft))
                else:
                    q2.append((neighbor, neither, fft, dac, both))

        dedup = defaultdict(lambda: (0, 0, 0, 0))
        for (node, neither, fft, dac, both) in q2:
            (neither_total, fft_total, dac_total, both_total) = dedup[node]
            dedup[node] = (neither + neither_total, fft + fft_total, dac + dac_total, both + both_total)

        q = []
        for (node, (n, f, d, b)) in dedup.items():
            q.append((node, n, f, d, b))

    return total


if __name__ == '__main__':
    main(sys.argv[1:])
