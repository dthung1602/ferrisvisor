#!/usr/bin/env python3

import sys
import time
from handle_sigterm import register

register(delay_sec=5)

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
        print("End process")
        exit(1)
