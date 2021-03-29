#!/usr/bin/env python3
# @raycast.schemaVersion 1
# @raycast.title Add shortlink
# @raycast.mode silent
# @raycast.packageName Shorty
# @raycast.icon 🔗
#
# Documentation:
# @raycast.author Matthew Gleich
# @raycast.authorURL https://mattglei.ch
# @raycast.argument1 { "type": "text", "placeholder": "URL" }
# @raycast.argument2 { "type": "text", "placeholder": "Name", "optional": true }

import requests
import json
import sys
import pyperclip

# token
shorty_token = ""
# e.g. https://clb.li
shorty_domain = ""

payload = {"url": sys.argv[1], "public": False}

if sys.argv[2] != "":
    payload["name"] = sys.argv[2]

name = requests.post(
    f"{shorty_domain}/api/link",
    headers={
        "Authorization": f"Bearer {shorty_token}",
        "Content-Type": "application/json",
    },
    data=json.dumps(payload),
).json()["data"]["name"]
link = f"{shorty_domain}/{name}"
pyperclip.copy(link)
print(f"Copied {link} to clipboard")
