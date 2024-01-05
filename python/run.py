import time

def main():
    nums = {}

    with open("measurements.txt") as f:
        s = time.time()

        for (i, line) in enumerate(f, start=1):
            if i % 10_000 == 0:
                print(f"{i} in {time.time() - s}s")

            id, n = line.rstrip().split(";", 1)
            n = float(n)

            try:
                num = nums[id]
                num["count"] += 1
                num["min"] = min(num["min"], n)
                num["max"] = max(num["max"], n)
                num["mean"] += (n - num["mean"]) / num["count"]
                nums[id] = num

            except KeyError:
                nums[id] = {"count": 1, "min": n, "mean": n, "max": n}

    for id, n in sorted(nums.items()):
        print(f"{id}={n.count}/{n.min}/{n.mean}/{n.max}")


if __name__ == "__main__":
    main()
