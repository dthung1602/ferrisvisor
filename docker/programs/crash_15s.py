#!/usr/bin/env python3
import sys
import time

print("Start process... Will crash soon")
for i in range(15):
    print("Counter: ", i)
    print("STDERR: counter ", i, file=sys.stderr)
    time.sleep(1)
raise Exception("Catastrophe failure")
