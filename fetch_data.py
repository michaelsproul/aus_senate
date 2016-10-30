#!/usr/bin/env python3

import os
import json
import hashlib
import requests

def sha256sum(f, blocksize=65536):
    hasher = hashlib.sha256()
    buf = f.read(blocksize)
    while len(buf) > 0:
        hasher.update(buf)
        buf = f.read(blocksize)
    return hasher.hexdigest()

def sha256_file(filename):
    with open(filename, "rb") as f:
        return sha256sum(f)

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

        checksum = sha256_file(filename)

        if checksum != info["sha256"]:
            print("Checksum error!\nExpected: {}\nDownloaded: {}".format(info["sha256"], checksum))

if __name__ == "__main__":
    main()
