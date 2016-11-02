#!/usr/bin/env python3

import os
import json
import subprocess as sp
import fetch_data
from datetime import datetime

cargo = ["cargo", "run", "--release", "--bin", "election2016", "--"]

def run():
    fetch_data.fetch()

    with open("states.json", "r") as f:
        states = json.load(f)

    data_dir = "data"

    candidate_ordering = os.path.join(data_dir, "candidate_ordering.csv")

    for (state, num_senators) in sorted(states.items()):
        print("Running election for {} at {}".format(state, timestamp()))

        state_csv = os.path.join(data_dir, "{}.csv".format(state))

        args = [candidate_ordering, state_csv, state, str(num_senators)]

        sp.call(cargo + args)

        print("Completed election for {} at {}".format(state, timestamp()))

def timestamp():
    return datetime.now().isoformat()

if __name__ == "__main__":
    run()
