#!/usr/bin/env python3

import sys
import time

print("Start process")

i = 0
while i < 15:
    try:
        print("Counter: ", i)
        time.sleep(1)
        if i % 10 == 0:
            print("STDERR Counter: ", i, file=sys.stderr)
        i += 1
    except:
        print("End process")

print("Process finished")
exit(2)
