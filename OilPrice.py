#!/usr/bin/env python3

import sys
import json
import urllib.request

OIL_PRICE_API = "https://api.oilpriceapi.com/v1/prices/latest"


def make_request(url):
    req = urllib.request.Request(url)
    req.add_header(
        "User-Agent",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2661.102 Safari/537.36",
    )
    req.add_header(
        "Authorization", "Token e386538ada1215da049b006aec48ab9c",
    )
    req.add_header("Content-Type", "application/json")
    return urllib.request.urlopen(req).read()


def main():
    raw = make_request(OIL_PRICE_API).decode()
    lastPrice = json.loads(raw)["data"]["price"]
    return lastPrice


if __name__ == "__main__":
    try:
        print(main(*sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)