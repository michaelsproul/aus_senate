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

def fetch(states):
    # Load data sources file
    sources_file = "data_sources.json"

    with open(sources_file, "r") as f:
        sources = json.load(f)

    # Prepare local cache of downloaded files
    dest_dir = "data"
    os.makedirs(dest_dir, mode=0o770, exist_ok=True)

    # Download/verify each required file
    for (filename, info) in sources.items():
        state = info.get("state")
        if state is not None and state not in states.keys():
            continue

        csv_file = os.path.join(dest_dir, filename)
        zip_file = os.path.join(dest_dir, filename + ".zip")

        csv_checksum = info.get("sha256")
        zip_checksum = info.get("zip-sha256")

        if os.path.exists(csv_file) and checksum_ok(csv_file, csv_checksum):
            print("Using cached version of {}".format(filename))
            continue

        if info.get("zipped") and os.path.exists(zip_file) and checksum_ok(zip_file, zip_checksum):
            print("Using cached zip version of {}".format(filename))
            extract_zip(zip_file, dest_dir, csv_file, csv_checksum, info)
            continue

        print("Downloading {}...".format(filename))
        res = requests.get(info["url"])

        if not res.ok:
            raise Exception("Download error!")

        if info.get("zipped"):
            save_file(zip_file, res.content, zip_checksum)
            extract_zip(zip_file, dest_dir, csv_file, csv_checksum, info)
        else:
            save_file(csv_file, res.content, csv_checksum)

def save_file(filename, content, checksum):
    with open(filename, "wb") as output:
        output.write(content)

    if not checksum_ok(filename, checksum):
        raise Exception("Downloaded file has the wrong checksum")

def extract_zip(zip_file, dest_dir, csv_file, csv_checksum, info):
    with zipfile.ZipFile(zip_file, "r") as z:
        z.extractall(dest_dir)

    temp_file = os.path.join(dest_dir, info["inner-file"])

    os.rename(temp_file, csv_file)

    if not checksum_ok(csv_file, csv_checksum):
        raise Exception("CSV file from within ZIP has the wrong checksum")

def checksum_ok(filename: str, checksum: str) -> bool:
    "Check that a file conforms to a SHA256 checksum"
    return checksum is None or sha256_file(filename) == checksum

if __name__ == "__main__":
    fetch()
