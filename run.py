#!/usr/bin/env python3

import os
import sys
import json
import subprocess as sp
import fetch_data
from datetime import datetime

cargo = ["cargo", "run", "--release", "--bin", "election2016", "--"]

def run():
    with open("states.json", "r") as f:
        states = json.load(f)

    # If states are specified on the command-line, just run elections for those states.
    if len(sys.argv) > 1:
        states = {s: n for (s, n) in states.items() if s in sys.argv[1:]}

    fetch_data.fetch(states)

    data_dir = "data"

    candidate_ids = os.path.join(data_dir, "candidate_ids.csv")
    candidate_ordering = os.path.join(data_dir, "candidate_ordering.csv")

    for (state, num_senators) in sorted(states.items()):
        print("Running election for {} at {}".format(state, timestamp()))

        state_csv = os.path.join(data_dir, "{}.csv".format(state))

        args = [candidate_ids, candidate_ordering, state_csv, state, str(num_senators)]

        sp.call(cargo + args)

        print("Completed election for {} at {}".format(state, timestamp()))

def timestamp():
    return datetime.now().isoformat()

if __name__ == "__main__":
    run()
