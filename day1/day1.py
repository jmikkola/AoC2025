#!/usr/bin/env python3


import sys

args = sys.argv[1:]
if args:
    fname = args[0]
else:
    fname = 'test'

with open(fname) as inf:
    lines = inf.readlines()


n_zeros = 0
dial = 50

for line in lines:
    line = line.replace('R', '').replace('L', '-')
    change = int(line)

    dial = (dial + change) % 100
    if dial == 0:
        n_zeros += 1

print(n_zeros)
