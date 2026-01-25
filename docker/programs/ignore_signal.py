#!/usr/bin/env python3

import sys
import time

print("Start process")

i = 0
while True:
    try:
        print("Counter: ", i)
        time.sleep(1)
        if i % 10 == 0:
            print("STDERR Counter: ", i, file=sys.stderr)
        i += 1
    except:
        print("Ctrl+C received. Ignore")
        print("Ctrl+C received. Ignore", file=sys.stderr)

