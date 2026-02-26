import termrender
import time

print("Hello Start", flush=True)
engine = termrender._termrender.Engine("test", 60.0, 60.0, 60.0)
print("Engine created!", flush=True)

for i in range(10):
    time.sleep(0.1)
    engine.poll_events()
    print(f"Polled {i}", flush=True)

print("End", flush=True)
