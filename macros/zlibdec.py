#!/usr/bin/env python3

# Decompress a zlib encoded file

from sys import argv
import os
import zlib

path = argv[1]

if not os.path.exists(path):
    print("Path does not exists!")
elif os.path.isfile(path):
    f = open(argv[1], 'rb')
    decomp = zlib.decompress(f.read())
    print(decomp)
else:
    for root, _, files in os.walk(path):
        for file in files:
            f = open(os.path.join(root,file), 'rb')
            decomp = zlib.decompress(f.read())
            print("OID: {}".format(file))
            print(decomp)
            print("---")
