#!/usr/bin/env python3

import os
import json
import requests

def main():
    sources_file = "data_sources.json"

    with open(sources_file, "r") as f:
        sources = json.load(f)

    # TODO: local caching.
    for (filename, info) in sources.items():
        print(filename)

        res = requests.get(info["url"])

        if not res.ok:
            print("Download error!")
            continue

        with open(filename, "w") as output:
            output.write(res.text)

        # TODO: sha256 checksums
        # checksum = sha256_file(filename)

        # print(checksum)

        # if checksum != info["sha256"]:
        #    print("Checksum error!")

if __name__ == "__main__":
    main()
