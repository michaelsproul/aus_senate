#!/usr/bin/env python3

import os
import json
import hashlib
import zipfile
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
    # Load data sources file
    sources_file = "data_sources.json"

    with open(sources_file, "r") as f:
        sources = json.load(f)

    # Prepare local cache of downloaded files
    dest_dir = "data"
    os.makedirs(dest_dir, mode=0o770, exist_ok=True)

    # Download/verify each required file
    for (filename, info) in sources.items():
        csv_file = os.path.join(dest_dir, filename)

        checksum = info.get("sha256")

        if os.path.exists(csv_file) and checksum_ok(csv_file, checksum):
            print("Using cached version of {}".format(filename))
            continue

        # TODO: maybe check for cached zip

        print("Downloading {}...".format(filename))
        res = requests.get(info["url"])

        if not res.ok:
            raise Exception("Download error!")

        if info.get("zipped"):
            save_zipped(csv_file, res.content, checksum, dest_dir, filename, info)
        else:
            save_nonzipped(csv_file, res.content, checksum)


def save_nonzipped(csv_file, content, checksum):
    with open(csv_file, "wb") as output:
        output.write(content)

    if not checksum_ok(csv_file, checksum):
        raise Exception("Downloaded file has the wrong checksum")

def save_zipped(csv_file, content, checksum, dest_dir, filename, info):
    zip_file = os.path.join(dest_dir, filename + ".zip")

    with open(zip_file, "wb") as z:
        z.write(content)

    if not checksum_ok(zip_file, info["zip-sha256"]):
        raise Exception("Downloaded zip file has the wrong checksum")

    with zipfile.ZipFile(zip_file, "r") as z:
        z.extractall(dest_dir)

    temp_file = os.path.join(dest_dir, info["inner-file"])

    os.rename(temp_file, csv_file)

    if not checksum_ok(csv_file, checksum):
        raise Exception("CSV file from within ZIP has the wrong checksum")

def checksum_ok(filename: str, checksum: str) -> bool:
    "Check that a file conforms to a SHA256 checksum"
    return checksum is None or sha256_file(filename) == checksum

if __name__ == "__main__":
    main()
