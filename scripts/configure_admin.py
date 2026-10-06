#!/usr/bin/env python3
"""Generate the admin environment variables without putting a password in shell history."""
import getpass
import json
import subprocess
from pathlib import Path

root = Path(__file__).resolve().parents[1]
password = getpass.getpass("Admin password (at least 12 characters): ")
if password != getpass.getpass("Repeat password: "):
    raise SystemExit("Passwords do not match.")
result = subprocess.run(
    ["cargo", "run", "--quiet", "--features", "server", "--example", "admin_credentials"],
    cwd=root, input=password, text=True, capture_output=True, check=False,
)
if result.returncode:
    raise SystemExit(result.stderr)
config = json.loads(result.stdout)
for name, value in config.items():
    print(f"{name}='{value}'")
