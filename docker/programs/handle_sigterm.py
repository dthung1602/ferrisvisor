import signal
import sys
import time


def register(exit_code=1, delay_sec=0):
    def handler(signum, frame):
        print('Signal handler called with signal', signum, flush=True)
        if exit_code is not None:
            time.sleep(delay_sec)
            sys.exit(exit_code)
        else:
            print("Igore signal", flush=True)

    signal.signal(signal.SIGTERM, handler)
